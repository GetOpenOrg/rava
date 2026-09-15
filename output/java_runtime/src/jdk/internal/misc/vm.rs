#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::stream::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::misc::*;

#[java_rta_macros::java_class(
    binary_name       = "jdk/internal/misc/VM",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "VM.java",
    inner_classes     = "java/lang/Thread$State:java/lang/Thread:State:16409;jdk/internal/misc/VM$BufferPoolsHolder:jdk/internal/misc/VM:BufferPoolsHolder:10;jdk/internal/misc/VM$BufferPool:jdk/internal/misc/VM:BufferPool:1545",
    all_supertypes    = "java/lang/Object;jdk/internal/misc/VM",
)]
#[derive(Clone, Default, PartialEq)]
pub struct VM;

impl VM {
    #[cfg_attr(any(), java_field(name = "JAVA_LANG_SYSTEM_INITED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
    // static field: JAVA_LANG_SYSTEM_INITED:I
    pub fn JAVA_LANG_SYSTEM_INITED() -> i32 {
        1
    }

    #[cfg_attr(any(), java_field(name = "MODULE_SYSTEM_INITED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
    // static field: MODULE_SYSTEM_INITED:I
    pub fn MODULE_SYSTEM_INITED() -> i32 {
        2
    }

    #[cfg_attr(any(), java_field(name = "SYSTEM_LOADER_INITIALIZING", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
    // static field: SYSTEM_LOADER_INITIALIZING:I
    pub fn SYSTEM_LOADER_INITIALIZING() -> i32 {
        3
    }

    #[cfg_attr(any(), java_field(name = "SYSTEM_BOOTED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4"))]
    // static field: SYSTEM_BOOTED:I
    pub fn SYSTEM_BOOTED() -> i32 {
        4
    }

    #[cfg_attr(any(), java_field(name = "SYSTEM_SHUTDOWN", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
    // static field: SYSTEM_SHUTDOWN:I
    pub fn SYSTEM_SHUTDOWN() -> i32 {
        5
    }

    #[cfg_attr(any(), java_field(name = "initLevel", descriptor = "I", access = "private", modifiers = "static volatile", is_static = true))]
    // static field: initLevel:I
    pub fn initLevel_field() -> i32 {
        panic!("stub: jdk/internal/misc/VM.initLevel:I")
    }

    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
    // static field: lock:Ljava/lang/Object;
    pub fn lock() -> Object {
        panic!("stub: jdk/internal/misc/VM.lock:Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_field(name = "javaLangInvokeInited", descriptor = "Z", access = "private", modifiers = "static", is_static = true))]
    // static field: javaLangInvokeInited:Z
    pub fn javaLangInvokeInited() -> bool {
        panic!("stub: jdk/internal/misc/VM.javaLangInvokeInited:Z")
    }

    #[cfg_attr(any(), java_field(name = "directMemory", descriptor = "J", access = "private", modifiers = "static", is_static = true))]
    // static field: directMemory:J
    pub fn directMemory() -> i64 {
        panic!("stub: jdk/internal/misc/VM.directMemory:J")
    }

    #[cfg_attr(any(), java_field(name = "pageAlignDirectMemory", descriptor = "Z", access = "private", modifiers = "static", is_static = true))]
    // static field: pageAlignDirectMemory:Z
    pub fn pageAlignDirectMemory() -> bool {
        panic!("stub: jdk/internal/misc/VM.pageAlignDirectMemory:Z")
    }

    #[cfg_attr(any(), java_field(name = "classFileMajorVersion", descriptor = "I", access = "private", modifiers = "static", is_static = true))]
    // static field: classFileMajorVersion:I
    pub fn classFileMajorVersion() -> i32 {
        panic!("stub: jdk/internal/misc/VM.classFileMajorVersion:I")
    }

    #[cfg_attr(any(), java_field(name = "classFileMinorVersion", descriptor = "I", access = "private", modifiers = "static", is_static = true))]
    // static field: classFileMinorVersion:I
    pub fn classFileMinorVersion() -> i32 {
        panic!("stub: jdk/internal/misc/VM.classFileMinorVersion:I")
    }

    #[cfg_attr(any(), java_field(name = "PREVIEW_MINOR_VERSION", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "65535"))]
    // static field: PREVIEW_MINOR_VERSION:I
    pub fn PREVIEW_MINOR_VERSION() -> i32 {
        65535
    }

    #[cfg_attr(any(), java_field(name = "savedProps", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;"))]
    // static field: savedProps:Ljava/util/Map;
    pub fn savedProps() -> Map<Object, Object> {
        panic!("stub: jdk/internal/misc/VM.savedProps:Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_field(name = "finalRefCount", descriptor = "I", access = "private", modifiers = "static volatile", is_static = true))]
    // static field: finalRefCount:I
    pub fn finalRefCount() -> i32 {
        panic!("stub: jdk/internal/misc/VM.finalRefCount:I")
    }

    #[cfg_attr(any(), java_field(name = "peakFinalRefCount", descriptor = "I", access = "private", modifiers = "static volatile", is_static = true))]
    // static field: peakFinalRefCount:I
    pub fn peakFinalRefCount() -> i32 {
        panic!("stub: jdk/internal/misc/VM.peakFinalRefCount:I")
    }

    #[cfg_attr(any(), java_field(name = "JVMTI_THREAD_STATE_ALIVE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
    // static field: JVMTI_THREAD_STATE_ALIVE:I
    pub fn JVMTI_THREAD_STATE_ALIVE() -> i32 {
        1
    }

    #[cfg_attr(any(), java_field(name = "JVMTI_THREAD_STATE_TERMINATED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
    // static field: JVMTI_THREAD_STATE_TERMINATED:I
    pub fn JVMTI_THREAD_STATE_TERMINATED() -> i32 {
        2
    }

    #[cfg_attr(any(), java_field(name = "JVMTI_THREAD_STATE_RUNNABLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4"))]
    // static field: JVMTI_THREAD_STATE_RUNNABLE:I
    pub fn JVMTI_THREAD_STATE_RUNNABLE() -> i32 {
        4
    }

    #[cfg_attr(any(), java_field(name = "JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1024"))]
    // static field: JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER:I
    pub fn JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER() -> i32 {
        1024
    }

    #[cfg_attr(any(), java_field(name = "JVMTI_THREAD_STATE_WAITING_INDEFINITELY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16"))]
    // static field: JVMTI_THREAD_STATE_WAITING_INDEFINITELY:I
    pub fn JVMTI_THREAD_STATE_WAITING_INDEFINITELY() -> i32 {
        16
    }

    #[cfg_attr(any(), java_field(name = "JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "32"))]
    // static field: JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT:I
    pub fn JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT() -> i32 {
        32
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: jdk/internal/misc/VM.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "initLevel", descriptor = "(I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn initLevel_i(value: i32) -> Result<()> {
        panic!("stub: jdk/internal/misc/VM.initLevel:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "initLevel", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn initLevel() -> Result<i32> {
        panic!("stub: jdk/internal/misc/VM.initLevel:()I")
    }

    #[cfg_attr(any(), java_method(name = "awaitInitLevel", descriptor = "(I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn awaitInitLevel(value: i32) -> Result<()> {
        panic!("stub: jdk/internal/misc/VM.awaitInitLevel:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "isModuleSystemInited", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isModuleSystemInited() -> Result<bool> {
        panic!("stub: jdk/internal/misc/VM.isModuleSystemInited:()Z")
    }

    #[cfg_attr(any(), java_method(name = "setJavaLangInvokeInited", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setJavaLangInvokeInited() -> Result<()> {
        panic!("stub: jdk/internal/misc/VM.setJavaLangInvokeInited:()V")
    }

    #[cfg_attr(any(), java_method(name = "isJavaLangInvokeInited", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isJavaLangInvokeInited() -> Result<bool> {
        panic!("stub: jdk/internal/misc/VM.isJavaLangInvokeInited:()Z")
    }

    #[cfg_attr(any(), java_method(name = "isBooted", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isBooted() -> Result<bool> {
        panic!("stub: jdk/internal/misc/VM.isBooted:()Z")
    }

    #[cfg_attr(any(), java_method(name = "shutdown", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn shutdown() -> Result<()> {
        panic!("stub: jdk/internal/misc/VM.shutdown:()V")
    }

    #[cfg_attr(any(), java_method(name = "isShutdown", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isShutdown() -> Result<bool> {
        panic!("stub: jdk/internal/misc/VM.isShutdown:()Z")
    }

    #[cfg_attr(any(), java_method(name = "maxDirectMemory", descriptor = "()J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn maxDirectMemory() -> Result<i64> {
        panic!("stub: jdk/internal/misc/VM.maxDirectMemory:()J")
    }

    #[cfg_attr(any(), java_method(name = "isDirectMemoryPageAligned", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isDirectMemoryPageAligned() -> Result<bool> {
        panic!("stub: jdk/internal/misc/VM.isDirectMemoryPageAligned:()Z")
    }

    #[cfg_attr(any(), java_method(name = "isSupportedClassFileVersion", descriptor = "(II)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isSupportedClassFileVersion(major: i32, minor: i32) -> Result<bool> {
        panic!("stub: jdk/internal/misc/VM.isSupportedClassFileVersion:(II)Z")
    }

    #[cfg_attr(any(), java_method(name = "isSupportedModuleDescriptorVersion", descriptor = "(II)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isSupportedModuleDescriptorVersion(major: i32, minor: i32) -> Result<bool> {
        panic!("stub: jdk/internal/misc/VM.isSupportedModuleDescriptorVersion:(II)Z")
    }

    #[cfg_attr(any(), java_method(name = "isSystemDomainLoader", descriptor = "(Ljava/lang/ClassLoader;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isSystemDomainLoader(loader: Object) -> Result<bool> {
        panic!("stub: jdk/internal/misc/VM.isSystemDomainLoader:(Ljava/lang/ClassLoader;)Z")
    }

    #[cfg_attr(any(), java_method(name = "getSavedProperty", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getSavedProperty(key: String) -> Result<String> {
        panic!("stub: jdk/internal/misc/VM.getSavedProperty:(Ljava/lang/String;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "getSavedProperties", descriptor = "()Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;"))]
    pub fn getSavedProperties() -> Result<Map<Object, Object>> {
        panic!("stub: jdk/internal/misc/VM.getSavedProperties:()Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "saveProperties", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;)V"))]
    pub fn saveProperties(props: Map<Object, Object>) -> Result<()> {
        panic!("stub: jdk/internal/misc/VM.saveProperties:(Ljava/util/Map;)V")
    }

    #[cfg_attr(any(), java_method(name = "initializeOSEnvironment", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn initializeOSEnvironment() -> Result<()> {
        panic!("stub: jdk/internal/misc/VM.initializeOSEnvironment:()V")
    }

    #[cfg_attr(any(), java_method(name = "getFinalRefCount", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getFinalRefCount() -> Result<i32> {
        panic!("stub: jdk/internal/misc/VM.getFinalRefCount:()I")
    }

    #[cfg_attr(any(), java_method(name = "getPeakFinalRefCount", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getPeakFinalRefCount() -> Result<i32> {
        panic!("stub: jdk/internal/misc/VM.getPeakFinalRefCount:()I")
    }

    #[cfg_attr(any(), java_method(name = "addFinalRefCount", descriptor = "(I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn addFinalRefCount(n: i32) -> Result<()> {
        panic!("stub: jdk/internal/misc/VM.addFinalRefCount:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "toThreadState", descriptor = "(I)Ljava/lang/Thread$State;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toThreadState(threadStatus: i32) -> Result<Thread_State> {
        panic!("stub: jdk/internal/misc/VM.toThreadState:(I)Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_method(name = "latestUserDefinedLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn latestUserDefinedLoader() -> Result<Object> {
        panic!("stub: jdk/internal/misc/VM.latestUserDefinedLoader:()Ljava/lang/ClassLoader;")
    }

    #[cfg_attr(any(), java_native(name = "latestUserDefinedLoader0", descriptor = "()Ljava/lang/ClassLoader;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn latestUserDefinedLoader0() -> Result<Object> {
        panic!("native: jdk/internal/misc/VM.latestUserDefinedLoader0:()Ljava/lang/ClassLoader;")
    }

    #[cfg_attr(any(), java_method(name = "isSetUID", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isSetUID() -> Result<bool> {
        panic!("stub: jdk/internal/misc/VM.isSetUID:()Z")
    }

    #[cfg_attr(any(), java_native(name = "getuid", descriptor = "()J", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getuid() -> Result<i64> {
        panic!("native: jdk/internal/misc/VM.getuid:()J")
    }

    #[cfg_attr(any(), java_native(name = "geteuid", descriptor = "()J", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn geteuid() -> Result<i64> {
        panic!("native: jdk/internal/misc/VM.geteuid:()J")
    }

    #[cfg_attr(any(), java_native(name = "getgid", descriptor = "()J", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getgid() -> Result<i64> {
        panic!("native: jdk/internal/misc/VM.getgid:()J")
    }

    #[cfg_attr(any(), java_native(name = "getegid", descriptor = "()J", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getegid() -> Result<i64> {
        panic!("native: jdk/internal/misc/VM.getegid:()J")
    }

    #[cfg_attr(any(), java_native(name = "getNanoTimeAdjustment", descriptor = "(J)J", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getNanoTimeAdjustment(arg0: i64) -> Result<i64> {
        panic!("native: jdk/internal/misc/VM.getNanoTimeAdjustment:(J)J")
    }

    #[cfg_attr(any(), java_native(name = "getRuntimeArguments", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getRuntimeArguments() -> Result<Rc<RefCell<Vec<String>>>> {
        panic!("native: jdk/internal/misc/VM.getRuntimeArguments:()[Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_native(name = "initialize", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn initialize() -> Result<()> {
        panic!("native: jdk/internal/misc/VM.initialize:()V")
    }

    #[cfg_attr(any(), java_method(name = "getBufferPools", descriptor = "()Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<Ljdk/internal/misc/VM$BufferPool;>;"))]
    pub fn getBufferPools() -> Result<List<Object>> {
        panic!("stub: jdk/internal/misc/VM.getBufferPools:()Ljava/util/List;")
    }
}
