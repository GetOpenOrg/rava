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
use crate::jdk::internal::misc::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::misc::Unsafe;
use crate::jdk::internal::util::ArraysSupport;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/misc/ScopedMemoryAccess"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ScopedMemoryAccess.java"]
    #[inner_classes     = "jdk/internal/misc/ScopedMemoryAccess$ScopedAccessError:jdk/internal/misc/ScopedMemoryAccess:ScopedAccessError:25;jdk/internal/vm/vector/VectorSupport$VectorSpecies:jdk/internal/vm/vector/VectorSupport:VectorSpecies:9;jdk/internal/vm/vector/VectorSupport$LoadOperation:jdk/internal/vm/vector/VectorSupport:LoadOperation:1545;jdk/internal/vm/vector/VectorSupport$Vector:jdk/internal/vm/vector/VectorSupport:Vector:9;jdk/internal/vm/vector/VectorSupport$VectorPayload:jdk/internal/vm/vector/VectorSupport:VectorPayload:9;jdk/internal/vm/vector/VectorSupport$VectorMask:jdk/internal/vm/vector/VectorSupport:VectorMask:9;jdk/internal/vm/vector/VectorSupport$LoadVectorMaskedOperation:jdk/internal/vm/vector/VectorSupport:LoadVectorMaskedOperation:1545;jdk/internal/vm/vector/VectorSupport$StoreVectorOperation:jdk/internal/vm/vector/VectorSupport:StoreVectorOperation:1545;jdk/internal/vm/vector/VectorSupport$StoreVectorMaskedOperation:jdk/internal/vm/vector/VectorSupport:StoreVectorMaskedOperation:1545;jdk/internal/misc/ScopedMemoryAccess$Scoped:jdk/internal/misc/ScopedMemoryAccess:Scoped:9736"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/misc/ScopedMemoryAccess"]

    pub struct ScopedMemoryAccess;

    impl ScopedMemoryAccess {
        #[cfg_attr(any(), java_field(name = "UNSAFE", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: UNSAFE:Ljdk/internal/misc/Unsafe;
        pub fn UNSAFE() -> Unsafe {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.UNSAFE:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "theScopedMemoryAccess", descriptor = "Ljdk/internal/misc/ScopedMemoryAccess;", access = "private", modifiers = "static final", is_static = true))]
        // static field: theScopedMemoryAccess:Ljdk/internal/misc/ScopedMemoryAccess;
        pub fn theScopedMemoryAccess() -> ScopedMemoryAccess {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.theScopedMemoryAccess:Ljdk/internal/misc/ScopedMemoryAccess;")
        }

        #[native]
        #[java_native(name = "registerNatives", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn registerNatives() -> Result<()> {
            panic!("native: jdk/internal/misc/ScopedMemoryAccess.registerNatives:()V")
        }

        #[java_method(name = "closeScope", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn closeScope(&self, session: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.closeScope:(Ljdk/internal/foreign/MemorySessionImpl;)Z")
        }

        #[native]
        #[java_native(name = "closeScope0", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;)Z", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn closeScope0(&self, arg0: Object) -> Result<bool> {
            panic!("native: jdk/internal/misc/ScopedMemoryAccess.closeScope0:(Ljdk/internal/foreign/MemorySessionImpl;)Z")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.<init>:()V")
        }

        #[java_method(name = "getScopedMemoryAccess", descriptor = "()Ljdk/internal/misc/ScopedMemoryAccess;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getScopedMemoryAccess() -> Result<ScopedMemoryAccess> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getScopedMemoryAccess:()Ljdk/internal/misc/ScopedMemoryAccess;")
        }

        #[java_method(name = "copyMemory", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyMemory(&self, srcSession: Object, dstSession: Object, srcBase: Object, srcOffset: i64, arg4: Object, destBase: i64, destOffset: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.copyMemory:(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJ)V")
        }

        #[java_method(name = "copyMemoryInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyMemoryInternal(&self, srcSession: Object, dstSession: Object, srcBase: Object, srcOffset: i64, arg4: Object, destBase: i64, destOffset: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.copyMemoryInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJ)V")
        }

        #[java_method(name = "copySwapMemory", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copySwapMemory(&self, srcSession: Object, dstSession: Object, srcBase: Object, srcOffset: i64, arg4: Object, destBase: i64, destOffset: i64, arg7: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.copySwapMemory:(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
        }

        #[java_method(name = "copySwapMemoryInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copySwapMemoryInternal(&self, srcSession: Object, dstSession: Object, srcBase: Object, srcOffset: i64, arg4: Object, destBase: i64, destOffset: i64, arg7: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.copySwapMemoryInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
        }

        #[java_method(name = "setMemory", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJB)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMemory(&self, session: Object, o: Object, offset: i64, arg3: i64, bytes: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.setMemory:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJB)V")
        }

        #[java_method(name = "setMemoryInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJB)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMemoryInternal(&self, session: Object, o: Object, offset: i64, arg3: i64, bytes: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.setMemoryInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJB)V")
        }

        #[java_method(name = "vectorizedMismatch", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn vectorizedMismatch(&self, aSession: Object, bSession: Object, a: Object, aOffset: i64, arg4: Object, b: i64, bOffset: i32, arg7: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.vectorizedMismatch:(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JII)I")
        }

        #[java_method(name = "vectorizedMismatchInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn vectorizedMismatchInternal(&self, aSession: Object, bSession: Object, a: Object, aOffset: i64, arg4: Object, b: i64, bOffset: i32, arg7: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.vectorizedMismatchInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JII)I")
        }

        #[java_method(name = "isLoaded", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;JZJ)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLoaded(&self, session: Object, address: i64, arg2: bool, isSync: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.isLoaded:(Ljdk/internal/foreign/MemorySessionImpl;JZJ)Z")
        }

        #[java_method(name = "isLoadedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;JZJ)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLoadedInternal(&self, session: Object, address: i64, arg2: bool, isSync: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.isLoadedInternal:(Ljdk/internal/foreign/MemorySessionImpl;JZJ)Z")
        }

        #[java_method(name = "load", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;JZJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn load(&self, session: Object, address: i64, arg2: bool, isSync: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.load:(Ljdk/internal/foreign/MemorySessionImpl;JZJ)V")
        }

        #[java_method(name = "loadInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;JZJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadInternal(&self, session: Object, address: i64, arg2: bool, isSync: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.loadInternal:(Ljdk/internal/foreign/MemorySessionImpl;JZJ)V")
        }

        #[java_method(name = "unload", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;JZJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unload(&self, session: Object, address: i64, arg2: bool, isSync: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.unload:(Ljdk/internal/foreign/MemorySessionImpl;JZJ)V")
        }

        #[java_method(name = "unloadInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;JZJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unloadInternal(&self, session: Object, address: i64, arg2: bool, isSync: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.unloadInternal:(Ljdk/internal/foreign/MemorySessionImpl;JZJ)V")
        }

        #[java_method(name = "force", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/io/FileDescriptor;JZJJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn force(&self, session: Object, fd: Object, address: i64, arg3: bool, isSync: i64, index: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.force:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/io/FileDescriptor;JZJJ)V")
        }

        #[java_method(name = "forceInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/io/FileDescriptor;JZJJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forceInternal(&self, session: Object, fd: Object, address: i64, arg3: bool, isSync: i64, index: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.forceInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/io/FileDescriptor;JZJJ)V")
        }

        #[java_method(name = "loadFromMemorySegment", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$VectorSpecies;Ljdk/internal/vm/vector/VectorSupport$LoadOperation;)Ljdk/internal/vm/vector/VectorSupport$Vector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<V:Ljdk/internal/vm/vector/VectorSupport$Vector<TE;>;E:Ljava/lang/Object;S:Ljdk/internal/vm/vector/VectorSupport$VectorSpecies<TE;>;>(Ljava/lang/Class<+TV;>;Ljava/lang/Class<TE;>;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JTS;Ljdk/internal/vm/vector/VectorSupport$LoadOperation<Ljdk/internal/foreign/AbstractMemorySegmentImpl;TV;TS;>;)TV;")]
        pub fn loadFromMemorySegment(vmClass: Object, e: Object, length: i32, msp: Object, offset: i64, arg5: Object, s: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.loadFromMemorySegment:(Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$VectorSpecies;Ljdk/internal/vm/vector/VectorSupport$LoadOperation;)Ljdk/internal/vm/vector/VectorSupport$Vector;")
        }

        #[java_method(name = "loadFromMemorySegmentScopedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$VectorSpecies;Ljdk/internal/vm/vector/VectorSupport$LoadOperation;)Ljdk/internal/vm/vector/VectorSupport$Vector;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<V:Ljdk/internal/vm/vector/VectorSupport$Vector<TE;>;E:Ljava/lang/Object;S:Ljdk/internal/vm/vector/VectorSupport$VectorSpecies<TE;>;>(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class<+TV;>;Ljava/lang/Class<TE;>;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JTS;Ljdk/internal/vm/vector/VectorSupport$LoadOperation<Ljdk/internal/foreign/AbstractMemorySegmentImpl;TV;TS;>;)TV;")]
        pub fn loadFromMemorySegmentScopedInternal(session: Object, vmClass: Object, e: Object, length: i32, msp: Object, offset: i64, arg6: Object, s: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.loadFromMemorySegmentScopedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$VectorSpecies;Ljdk/internal/vm/vector/VectorSupport$LoadOperation;)Ljdk/internal/vm/vector/VectorSupport$Vector;")
        }

        #[java_method(name = "loadFromMemorySegmentMasked", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$VectorMask;Ljdk/internal/vm/vector/VectorSupport$VectorSpecies;ILjdk/internal/vm/vector/VectorSupport$LoadVectorMaskedOperation;)Ljdk/internal/vm/vector/VectorSupport$Vector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<V:Ljdk/internal/vm/vector/VectorSupport$Vector<TE;>;E:Ljava/lang/Object;S:Ljdk/internal/vm/vector/VectorSupport$VectorSpecies<TE;>;M:Ljdk/internal/vm/vector/VectorSupport$VectorMask<TE;>;>(Ljava/lang/Class<+TV;>;Ljava/lang/Class<TM;>;Ljava/lang/Class<TE;>;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JTM;TS;ILjdk/internal/vm/vector/VectorSupport$LoadVectorMaskedOperation<Ljdk/internal/foreign/AbstractMemorySegmentImpl;TV;TS;TM;>;)TV;")]
        pub fn loadFromMemorySegmentMasked(vmClass: Object, maskClass: Object, e: Object, length: i32, msp: Object, offset: i64, arg6: Object, m: Object, s: i32, offsetInRange: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.loadFromMemorySegmentMasked:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$VectorMask;Ljdk/internal/vm/vector/VectorSupport$VectorSpecies;ILjdk/internal/vm/vector/VectorSupport$LoadVectorMaskedOperation;)Ljdk/internal/vm/vector/VectorSupport$Vector;")
        }

        #[java_method(name = "loadFromMemorySegmentMaskedScopedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$VectorMask;Ljdk/internal/vm/vector/VectorSupport$VectorSpecies;ILjdk/internal/vm/vector/VectorSupport$LoadVectorMaskedOperation;)Ljdk/internal/vm/vector/VectorSupport$Vector;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<V:Ljdk/internal/vm/vector/VectorSupport$Vector<TE;>;E:Ljava/lang/Object;S:Ljdk/internal/vm/vector/VectorSupport$VectorSpecies<TE;>;M:Ljdk/internal/vm/vector/VectorSupport$VectorMask<TE;>;>(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class<+TV;>;Ljava/lang/Class<TM;>;Ljava/lang/Class<TE;>;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JTM;TS;ILjdk/internal/vm/vector/VectorSupport$LoadVectorMaskedOperation<Ljdk/internal/foreign/AbstractMemorySegmentImpl;TV;TS;TM;>;)TV;")]
        pub fn loadFromMemorySegmentMaskedScopedInternal(session: Object, vmClass: Object, maskClass: Object, e: Object, length: i32, msp: Object, offset: i64, arg7: Object, m: Object, s: i32, offsetInRange: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.loadFromMemorySegmentMaskedScopedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$VectorMask;Ljdk/internal/vm/vector/VectorSupport$VectorSpecies;ILjdk/internal/vm/vector/VectorSupport$LoadVectorMaskedOperation;)Ljdk/internal/vm/vector/VectorSupport$Vector;")
        }

        #[java_method(name = "storeIntoMemorySegment", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/vm/vector/VectorSupport$Vector;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorOperation;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<V:Ljdk/internal/vm/vector/VectorSupport$Vector<TE;>;E:Ljava/lang/Object;>(Ljava/lang/Class<+TV;>;Ljava/lang/Class<TE;>;ITV;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorOperation<Ljdk/internal/foreign/AbstractMemorySegmentImpl;TV;>;)V")]
        pub fn storeIntoMemorySegment(vmClass: Object, e: Object, length: i32, v: Object, msp: Object, offset: i64, arg6: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.storeIntoMemorySegment:(Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/vm/vector/VectorSupport$Vector;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorOperation;)V")
        }

        #[java_method(name = "storeIntoMemorySegmentScopedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/vm/vector/VectorSupport$Vector;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorOperation;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<V:Ljdk/internal/vm/vector/VectorSupport$Vector<TE;>;E:Ljava/lang/Object;>(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class<+TV;>;Ljava/lang/Class<TE;>;ITV;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorOperation<Ljdk/internal/foreign/AbstractMemorySegmentImpl;TV;>;)V")]
        pub fn storeIntoMemorySegmentScopedInternal(session: Object, vmClass: Object, e: Object, length: i32, v: Object, msp: Object, offset: i64, arg7: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.storeIntoMemorySegmentScopedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/vm/vector/VectorSupport$Vector;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorOperation;)V")
        }

        #[java_method(name = "storeIntoMemorySegmentMasked", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/vm/vector/VectorSupport$Vector;Ljdk/internal/vm/vector/VectorSupport$VectorMask;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorMaskedOperation;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<V:Ljdk/internal/vm/vector/VectorSupport$Vector<TE;>;E:Ljava/lang/Object;M:Ljdk/internal/vm/vector/VectorSupport$VectorMask<TE;>;>(Ljava/lang/Class<+TV;>;Ljava/lang/Class<TM;>;Ljava/lang/Class<TE;>;ITV;TM;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorMaskedOperation<Ljdk/internal/foreign/AbstractMemorySegmentImpl;TV;TM;>;)V")]
        pub fn storeIntoMemorySegmentMasked(vmClass: Object, maskClass: Object, e: Object, length: i32, v: Object, m: Object, msp: Object, offset: i64, arg8: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.storeIntoMemorySegmentMasked:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/vm/vector/VectorSupport$Vector;Ljdk/internal/vm/vector/VectorSupport$VectorMask;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorMaskedOperation;)V")
        }

        #[java_method(name = "storeIntoMemorySegmentMaskedScopedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/vm/vector/VectorSupport$Vector;Ljdk/internal/vm/vector/VectorSupport$VectorMask;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorMaskedOperation;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<V:Ljdk/internal/vm/vector/VectorSupport$Vector<TE;>;E:Ljava/lang/Object;M:Ljdk/internal/vm/vector/VectorSupport$VectorMask<TE;>;>(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class<+TV;>;Ljava/lang/Class<TM;>;Ljava/lang/Class<TE;>;ITV;TM;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorMaskedOperation<Ljdk/internal/foreign/AbstractMemorySegmentImpl;TV;TM;>;)V")]
        pub fn storeIntoMemorySegmentMaskedScopedInternal(session: Object, vmClass: Object, maskClass: Object, e: Object, length: i32, v: Object, m: Object, msp: Object, offset: i64, arg9: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.storeIntoMemorySegmentMaskedScopedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;ILjdk/internal/vm/vector/VectorSupport$Vector;Ljdk/internal/vm/vector/VectorSupport$VectorMask;Ljdk/internal/foreign/AbstractMemorySegmentImpl;JLjdk/internal/vm/vector/VectorSupport$StoreVectorMaskedOperation;)V")
        }

        #[java_method(name = "getByte", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByte(&self, session: Object, base: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getByte:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B")
        }

        #[java_method(name = "getByteInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByteInternal(&self, session: Object, base: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getByteInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B")
        }

        #[java_method(name = "putByte", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByte(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putByte:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "putByteInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByteInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putByteInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "getByteVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByteVolatile(&self, session: Object, base: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getByteVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B")
        }

        #[java_method(name = "getByteVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByteVolatileInternal(&self, session: Object, base: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getByteVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B")
        }

        #[java_method(name = "putByteVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByteVolatile(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putByteVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "putByteVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByteVolatileInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putByteVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "getByteAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByteAcquire(&self, session: Object, base: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getByteAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B")
        }

        #[java_method(name = "getByteAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByteAcquireInternal(&self, session: Object, base: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getByteAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B")
        }

        #[java_method(name = "putByteRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByteRelease(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putByteRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "putByteReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByteReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putByteReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "getByteOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByteOpaque(&self, session: Object, base: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getByteOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B")
        }

        #[java_method(name = "getByteOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByteOpaqueInternal(&self, session: Object, base: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getByteOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)B")
        }

        #[java_method(name = "putByteOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByteOpaque(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putByteOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "putByteOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByteOpaqueInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putByteOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "getAndAddByte", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddByte(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddByte:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndAddByteInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddByteInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddByteInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndAddByteAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddByteAcquire(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddByteAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndAddByteAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddByteAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddByteAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndAddByteRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddByteRelease(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddByteRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndAddByteReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddByteReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddByteReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseOrByte", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrByte(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrByte:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseOrByteInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrByteInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrByteInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseOrByteAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrByteAcquire(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrByteAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseOrByteAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrByteAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrByteAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseOrByteRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrByteRelease(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrByteRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseOrByteReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrByteReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrByteReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseAndByte", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndByte(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndByte:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseAndByteInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndByteInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndByteInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseAndByteAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndByteAcquire(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndByteAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseAndByteAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndByteAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndByteAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseAndByteRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndByteRelease(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndByteRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseAndByteReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndByteReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndByteReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseXorByte", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorByte(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorByte:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseXorByteInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorByteInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorByteInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseXorByteAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorByteAcquire(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorByteAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseXorByteAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorByteAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorByteAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseXorByteRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorByteRelease(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorByteRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseXorByteReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorByteReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorByteReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getShort", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShort(&self, session: Object, base: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShort:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S")
        }

        #[java_method(name = "getShortInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortInternal(&self, session: Object, base: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShortInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S")
        }

        #[java_method(name = "putShort", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShort(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShort:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "putShortInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShortInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "getShortUnaligned", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortUnaligned(&self, session: Object, base: Object, offset: i64, arg3: bool) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShortUnaligned:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)S")
        }

        #[java_method(name = "getShortUnalignedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortUnalignedInternal(&self, session: Object, base: Object, offset: i64, arg3: bool) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShortUnalignedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)S")
        }

        #[java_method(name = "putShortUnaligned", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JSZ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortUnaligned(&self, session: Object, base: Object, offset: i64, arg3: i16, value: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShortUnaligned:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JSZ)V")
        }

        #[java_method(name = "putShortUnalignedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JSZ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortUnalignedInternal(&self, session: Object, base: Object, offset: i64, arg3: i16, value: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShortUnalignedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JSZ)V")
        }

        #[java_method(name = "getShortVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortVolatile(&self, session: Object, base: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShortVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S")
        }

        #[java_method(name = "getShortVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortVolatileInternal(&self, session: Object, base: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShortVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S")
        }

        #[java_method(name = "putShortVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortVolatile(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShortVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "putShortVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortVolatileInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShortVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "getShortAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortAcquire(&self, session: Object, base: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShortAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S")
        }

        #[java_method(name = "getShortAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortAcquireInternal(&self, session: Object, base: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShortAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S")
        }

        #[java_method(name = "putShortRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortRelease(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShortRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "putShortReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShortReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "getShortOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortOpaque(&self, session: Object, base: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShortOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S")
        }

        #[java_method(name = "getShortOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortOpaqueInternal(&self, session: Object, base: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getShortOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)S")
        }

        #[java_method(name = "putShortOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortOpaque(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShortOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "putShortOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortOpaqueInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putShortOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "getAndAddShort", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddShort(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddShort:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndAddShortInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddShortInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddShortInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndAddShortAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddShortAcquire(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddShortAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndAddShortAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddShortAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddShortAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndAddShortRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddShortRelease(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddShortRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndAddShortReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddShortReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddShortReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseOrShort", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrShort(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrShort:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseOrShortInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrShortInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrShortInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseOrShortAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrShortAcquire(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrShortAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseOrShortAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrShortAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrShortAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseOrShortRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrShortRelease(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrShortRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseOrShortReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrShortReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrShortReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseAndShort", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndShort(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndShort:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseAndShortInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndShortInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndShortInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseAndShortAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndShortAcquire(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndShortAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseAndShortAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndShortAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndShortAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseAndShortRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndShortRelease(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndShortRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseAndShortReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndShortReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndShortReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseXorShort", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorShort(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorShort:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseXorShortInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorShortInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorShortInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseXorShortAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorShortAcquire(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorShortAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseXorShortAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorShortAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorShortAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseXorShortRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorShortRelease(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorShortRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseXorShortReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorShortReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorShortReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getChar", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChar(&self, session: Object, base: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getChar:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C")
        }

        #[java_method(name = "getCharInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharInternal(&self, session: Object, base: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getCharInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C")
        }

        #[java_method(name = "putChar", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putChar(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putChar:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "putCharInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putCharInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "getCharUnaligned", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharUnaligned(&self, session: Object, base: Object, offset: i64, arg3: bool) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getCharUnaligned:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)C")
        }

        #[java_method(name = "getCharUnalignedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharUnalignedInternal(&self, session: Object, base: Object, offset: i64, arg3: bool) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getCharUnalignedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)C")
        }

        #[java_method(name = "putCharUnaligned", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JCZ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharUnaligned(&self, session: Object, base: Object, offset: i64, arg3: u16, value: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putCharUnaligned:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JCZ)V")
        }

        #[java_method(name = "putCharUnalignedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JCZ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharUnalignedInternal(&self, session: Object, base: Object, offset: i64, arg3: u16, value: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putCharUnalignedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JCZ)V")
        }

        #[java_method(name = "getCharVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharVolatile(&self, session: Object, base: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getCharVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C")
        }

        #[java_method(name = "getCharVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharVolatileInternal(&self, session: Object, base: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getCharVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C")
        }

        #[java_method(name = "putCharVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharVolatile(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putCharVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "putCharVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharVolatileInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putCharVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "getCharAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharAcquire(&self, session: Object, base: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getCharAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C")
        }

        #[java_method(name = "getCharAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharAcquireInternal(&self, session: Object, base: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getCharAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C")
        }

        #[java_method(name = "putCharRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharRelease(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putCharRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "putCharReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putCharReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "getCharOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharOpaque(&self, session: Object, base: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getCharOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C")
        }

        #[java_method(name = "getCharOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharOpaqueInternal(&self, session: Object, base: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getCharOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)C")
        }

        #[java_method(name = "putCharOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharOpaque(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putCharOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "putCharOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharOpaqueInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putCharOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "getAndAddChar", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddChar(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddChar:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndAddCharInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddCharInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddCharInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndAddCharAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddCharAcquire(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddCharAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndAddCharAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddCharAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddCharAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndAddCharRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddCharRelease(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddCharRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndAddCharReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddCharReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddCharReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseOrChar", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrChar(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrChar:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseOrCharInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrCharInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrCharInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseOrCharAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrCharAcquire(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrCharAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseOrCharAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrCharAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrCharAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseOrCharRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrCharRelease(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrCharRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseOrCharReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrCharReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrCharReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseAndChar", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndChar(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndChar:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseAndCharInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndCharInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndCharInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseAndCharAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndCharAcquire(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndCharAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseAndCharAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndCharAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndCharAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseAndCharRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndCharRelease(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndCharRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseAndCharReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndCharReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndCharReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseXorChar", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorChar(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorChar:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseXorCharInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorCharInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorCharInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseXorCharAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorCharAcquire(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorCharAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseXorCharAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorCharAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorCharAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseXorCharRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorCharRelease(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorCharRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseXorCharReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorCharReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorCharReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInt(&self, session: Object, base: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I")
        }

        #[java_method(name = "getIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntInternal(&self, session: Object, base: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I")
        }

        #[java_method(name = "putInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putInt(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "putIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "getIntUnaligned", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntUnaligned(&self, session: Object, base: Object, offset: i64, arg3: bool) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getIntUnaligned:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)I")
        }

        #[java_method(name = "getIntUnalignedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntUnalignedInternal(&self, session: Object, base: Object, offset: i64, arg3: bool) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getIntUnalignedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)I")
        }

        #[java_method(name = "putIntUnaligned", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JIZ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntUnaligned(&self, session: Object, base: Object, offset: i64, arg3: i32, value: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putIntUnaligned:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JIZ)V")
        }

        #[java_method(name = "putIntUnalignedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JIZ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntUnalignedInternal(&self, session: Object, base: Object, offset: i64, arg3: i32, value: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putIntUnalignedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JIZ)V")
        }

        #[java_method(name = "getIntVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntVolatile(&self, session: Object, base: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getIntVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I")
        }

        #[java_method(name = "getIntVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntVolatileInternal(&self, session: Object, base: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getIntVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I")
        }

        #[java_method(name = "putIntVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntVolatile(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putIntVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "putIntVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntVolatileInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putIntVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "getIntAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntAcquire(&self, session: Object, base: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getIntAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I")
        }

        #[java_method(name = "getIntAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntAcquireInternal(&self, session: Object, base: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getIntAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I")
        }

        #[java_method(name = "putIntRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntRelease(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putIntRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "putIntReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putIntReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "getIntOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntOpaque(&self, session: Object, base: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getIntOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I")
        }

        #[java_method(name = "getIntOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntOpaqueInternal(&self, session: Object, base: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getIntOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)I")
        }

        #[java_method(name = "putIntOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntOpaque(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putIntOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "putIntOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntOpaqueInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putIntOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "compareAndSetInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetInt(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndSetInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "compareAndSetIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetIntInternal(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndSetIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "compareAndExchangeInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeInt(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I")
        }

        #[java_method(name = "compareAndExchangeIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeIntInternal(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I")
        }

        #[java_method(name = "compareAndExchangeIntAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeIntAcquire(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeIntAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I")
        }

        #[java_method(name = "compareAndExchangeIntAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeIntAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeIntAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I")
        }

        #[java_method(name = "compareAndExchangeIntRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeIntRelease(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeIntRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I")
        }

        #[java_method(name = "compareAndExchangeIntReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeIntReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeIntReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)I")
        }

        #[java_method(name = "weakCompareAndSetIntPlain", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntPlain(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetIntPlain:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetIntPlainInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntPlainInternal(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetIntPlainInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetInt(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntInternal(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetIntAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntAcquire(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetIntAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetIntAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetIntAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetIntRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntRelease(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetIntRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetIntReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetIntReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "getAndSetInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetInt(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndSetIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetIntInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndSetIntAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetIntAcquire(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetIntAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndSetIntAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetIntAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetIntAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndSetIntRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetIntRelease(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetIntRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndSetIntReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetIntReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetIntReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndAddInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddInt(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndAddIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddIntInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndAddIntAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddIntAcquire(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddIntAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndAddIntAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddIntAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddIntAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndAddIntRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddIntRelease(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddIntRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndAddIntReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddIntReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddIntReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseOrInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrInt(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseOrIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrIntInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseOrIntAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrIntAcquire(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrIntAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseOrIntAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrIntAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrIntAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseOrIntRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrIntRelease(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrIntRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseOrIntReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrIntReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrIntReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseAndInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndInt(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseAndIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndIntInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseAndIntAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndIntAcquire(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndIntAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseAndIntAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndIntAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndIntAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseAndIntRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndIntRelease(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndIntRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseAndIntReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndIntReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndIntReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseXorInt", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorInt(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorInt:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseXorIntInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorIntInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorIntInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseXorIntAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorIntAcquire(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorIntAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseXorIntAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorIntAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorIntAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseXorIntRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorIntRelease(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorIntRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseXorIntReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorIntReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorIntReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self, session: Object, base: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J")
        }

        #[java_method(name = "getLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongInternal(&self, session: Object, base: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J")
        }

        #[java_method(name = "putLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLong(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "putLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "getLongUnaligned", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongUnaligned(&self, session: Object, base: Object, offset: i64, arg3: bool) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLongUnaligned:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)J")
        }

        #[java_method(name = "getLongUnalignedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongUnalignedInternal(&self, session: Object, base: Object, offset: i64, arg3: bool) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLongUnalignedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JZ)J")
        }

        #[java_method(name = "putLongUnaligned", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJZ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongUnaligned(&self, session: Object, base: Object, offset: i64, arg3: i64, value: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLongUnaligned:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJZ)V")
        }

        #[java_method(name = "putLongUnalignedInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJZ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongUnalignedInternal(&self, session: Object, base: Object, offset: i64, arg3: i64, value: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLongUnalignedInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJZ)V")
        }

        #[java_method(name = "getLongVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongVolatile(&self, session: Object, base: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLongVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J")
        }

        #[java_method(name = "getLongVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongVolatileInternal(&self, session: Object, base: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLongVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J")
        }

        #[java_method(name = "putLongVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongVolatile(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLongVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "putLongVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongVolatileInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLongVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "getLongAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongAcquire(&self, session: Object, base: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLongAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J")
        }

        #[java_method(name = "getLongAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongAcquireInternal(&self, session: Object, base: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLongAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J")
        }

        #[java_method(name = "putLongRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongRelease(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLongRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "putLongReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLongReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "getLongOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongOpaque(&self, session: Object, base: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLongOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J")
        }

        #[java_method(name = "getLongOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongOpaqueInternal(&self, session: Object, base: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getLongOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)J")
        }

        #[java_method(name = "putLongOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongOpaque(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLongOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "putLongOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongOpaqueInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putLongOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "compareAndSetLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetLong(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndSetLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "compareAndSetLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetLongInternal(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndSetLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "compareAndExchangeLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeLong(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J")
        }

        #[java_method(name = "compareAndExchangeLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeLongInternal(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J")
        }

        #[java_method(name = "compareAndExchangeLongAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeLongAcquire(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeLongAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J")
        }

        #[java_method(name = "compareAndExchangeLongAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeLongAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeLongAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J")
        }

        #[java_method(name = "compareAndExchangeLongRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeLongRelease(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeLongRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J")
        }

        #[java_method(name = "compareAndExchangeLongReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeLongReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeLongReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)J")
        }

        #[java_method(name = "weakCompareAndSetLongPlain", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongPlain(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetLongPlain:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLongPlainInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongPlainInternal(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetLongPlainInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLong(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongInternal(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLongAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongAcquire(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetLongAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLongAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetLongAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLongRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongRelease(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetLongRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLongReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetLongReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "getAndSetLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetLong(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndSetLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetLongInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndSetLongAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetLongAcquire(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetLongAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndSetLongAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetLongAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetLongAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndSetLongRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetLongRelease(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetLongRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndSetLongReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetLongReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetLongReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndAddLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddLong(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndAddLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddLongInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndAddLongAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddLongAcquire(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddLongAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndAddLongAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddLongAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddLongAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndAddLongRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddLongRelease(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddLongRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndAddLongReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddLongReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddLongReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseOrLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrLong(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseOrLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrLongInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseOrLongAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrLongAcquire(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrLongAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseOrLongAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrLongAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrLongAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseOrLongRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrLongRelease(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrLongRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseOrLongReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrLongReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseOrLongReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseAndLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndLong(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseAndLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndLongInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseAndLongAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndLongAcquire(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndLongAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseAndLongAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndLongAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndLongAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseAndLongRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndLongRelease(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndLongRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseAndLongReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndLongReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseAndLongReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseXorLong", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorLong(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorLong:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseXorLongInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorLongInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorLongInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseXorLongAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorLongAcquire(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorLongAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseXorLongAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorLongAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorLongAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseXorLongRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorLongRelease(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorLongRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseXorLongReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorLongReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndBitwiseXorLongReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getFloat", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloat(&self, session: Object, base: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getFloat:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F")
        }

        #[java_method(name = "getFloatInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatInternal(&self, session: Object, base: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getFloatInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F")
        }

        #[java_method(name = "putFloat", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloat(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putFloat:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "putFloatInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putFloatInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "getFloatVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatVolatile(&self, session: Object, base: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getFloatVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F")
        }

        #[java_method(name = "getFloatVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatVolatileInternal(&self, session: Object, base: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getFloatVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F")
        }

        #[java_method(name = "putFloatVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatVolatile(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putFloatVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "putFloatVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatVolatileInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putFloatVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "getFloatAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatAcquire(&self, session: Object, base: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getFloatAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F")
        }

        #[java_method(name = "getFloatAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatAcquireInternal(&self, session: Object, base: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getFloatAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F")
        }

        #[java_method(name = "putFloatRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatRelease(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putFloatRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "putFloatReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putFloatReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "getFloatOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatOpaque(&self, session: Object, base: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getFloatOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F")
        }

        #[java_method(name = "getFloatOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatOpaqueInternal(&self, session: Object, base: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getFloatOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)F")
        }

        #[java_method(name = "putFloatOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatOpaque(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putFloatOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "putFloatOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatOpaqueInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putFloatOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "compareAndSetFloat", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetFloat(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndSetFloat:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "compareAndSetFloatInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetFloatInternal(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndSetFloatInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "compareAndExchangeFloat", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeFloat(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeFloat:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F")
        }

        #[java_method(name = "compareAndExchangeFloatInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeFloatInternal(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeFloatInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F")
        }

        #[java_method(name = "compareAndExchangeFloatAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeFloatAcquire(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeFloatAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F")
        }

        #[java_method(name = "compareAndExchangeFloatAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeFloatAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeFloatAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F")
        }

        #[java_method(name = "compareAndExchangeFloatRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeFloatRelease(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeFloatRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F")
        }

        #[java_method(name = "compareAndExchangeFloatReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeFloatReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeFloatReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)F")
        }

        #[java_method(name = "weakCompareAndSetFloatPlain", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatPlain(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetFloatPlain:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloatPlainInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatPlainInternal(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetFloatPlainInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloat", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloat(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetFloat:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloatInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatInternal(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetFloatInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloatAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatAcquire(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetFloatAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloatAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetFloatAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloatRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatRelease(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetFloatRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloatReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetFloatReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "getAndSetFloat", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetFloat(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetFloat:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndSetFloatInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetFloatInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetFloatInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndSetFloatAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetFloatAcquire(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetFloatAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndSetFloatAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetFloatAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetFloatAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndSetFloatRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetFloatRelease(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetFloatRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndSetFloatReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetFloatReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetFloatReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndAddFloat", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddFloat(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddFloat:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndAddFloatInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddFloatInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddFloatInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndAddFloatAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddFloatAcquire(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddFloatAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndAddFloatAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddFloatAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddFloatAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndAddFloatRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddFloatRelease(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddFloatRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndAddFloatReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddFloatReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddFloatReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getDouble", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDouble(&self, session: Object, base: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getDouble:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D")
        }

        #[java_method(name = "getDoubleInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleInternal(&self, session: Object, base: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getDoubleInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D")
        }

        #[java_method(name = "putDouble", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDouble(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putDouble:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "putDoubleInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putDoubleInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "getDoubleVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleVolatile(&self, session: Object, base: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getDoubleVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D")
        }

        #[java_method(name = "getDoubleVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleVolatileInternal(&self, session: Object, base: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getDoubleVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D")
        }

        #[java_method(name = "putDoubleVolatile", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleVolatile(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putDoubleVolatile:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "putDoubleVolatileInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleVolatileInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putDoubleVolatileInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "getDoubleAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleAcquire(&self, session: Object, base: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getDoubleAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D")
        }

        #[java_method(name = "getDoubleAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleAcquireInternal(&self, session: Object, base: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getDoubleAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D")
        }

        #[java_method(name = "putDoubleRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleRelease(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putDoubleRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "putDoubleReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putDoubleReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "getDoubleOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleOpaque(&self, session: Object, base: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getDoubleOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D")
        }

        #[java_method(name = "getDoubleOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleOpaqueInternal(&self, session: Object, base: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getDoubleOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;J)D")
        }

        #[java_method(name = "putDoubleOpaque", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleOpaque(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putDoubleOpaque:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "putDoubleOpaqueInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleOpaqueInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.putDoubleOpaqueInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "compareAndSetDouble", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetDouble(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndSetDouble:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "compareAndSetDoubleInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetDoubleInternal(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndSetDoubleInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "compareAndExchangeDouble", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeDouble(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeDouble:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D")
        }

        #[java_method(name = "compareAndExchangeDoubleInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeDoubleInternal(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeDoubleInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D")
        }

        #[java_method(name = "compareAndExchangeDoubleAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeDoubleAcquire(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeDoubleAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D")
        }

        #[java_method(name = "compareAndExchangeDoubleAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeDoubleAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeDoubleAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D")
        }

        #[java_method(name = "compareAndExchangeDoubleRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeDoubleRelease(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeDoubleRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D")
        }

        #[java_method(name = "compareAndExchangeDoubleReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeDoubleReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.compareAndExchangeDoubleReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)D")
        }

        #[java_method(name = "weakCompareAndSetDoublePlain", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoublePlain(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetDoublePlain:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDoublePlainInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoublePlainInternal(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetDoublePlainInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDouble", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDouble(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetDouble:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDoubleInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoubleInternal(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetDoubleInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDoubleAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoubleAcquire(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetDoubleAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDoubleAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoubleAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetDoubleAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDoubleRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoubleRelease(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetDoubleRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDoubleReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoubleReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.weakCompareAndSetDoubleReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "getAndSetDouble", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetDouble(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetDouble:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndSetDoubleInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetDoubleInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetDoubleInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndSetDoubleAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetDoubleAcquire(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetDoubleAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndSetDoubleAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetDoubleAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetDoubleAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndSetDoubleRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetDoubleRelease(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetDoubleRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndSetDoubleReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetDoubleReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndSetDoubleReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndAddDouble", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddDouble(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddDouble:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndAddDoubleInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddDoubleInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddDoubleInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndAddDoubleAcquire", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddDoubleAcquire(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddDoubleAcquire:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndAddDoubleAcquireInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddDoubleAcquireInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddDoubleAcquireInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndAddDoubleRelease", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddDoubleRelease(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddDoubleRelease:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndAddDoubleReleaseInternal", descriptor = "(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddDoubleReleaseInternal(&self, session: Object, base: Object, offset: i64, arg3: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/ScopedMemoryAccess.getAndAddDoubleReleaseInternal:(Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JD)D")
        }
    }
}
