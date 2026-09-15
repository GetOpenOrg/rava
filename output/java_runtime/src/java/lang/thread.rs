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
use crate::jdk::internal::misc::VM;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Thread",
    super_class       = "java/lang/Object",
    interfaces        = "java/lang/Runnable",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Thread.java",
    inner_classes     = "jdk/internal/vm/ScopedValueContainer$BindingsSnapshot:jdk/internal/vm/ScopedValueContainer:BindingsSnapshot:25;java/lang/Thread$FieldHolder:java/lang/Thread:FieldHolder:10;java/lang/Thread$ThreadIdentifiers:java/lang/Thread:ThreadIdentifiers:10;java/lang/ThreadLocal$ThreadLocalMap:java/lang/ThreadLocal:ThreadLocalMap:8;java/lang/Thread$Constants:java/lang/Thread:Constants:10;java/lang/ThreadBuilders$PlatformThreadBuilder:java/lang/ThreadBuilders:PlatformThreadBuilder:24;java/lang/ThreadBuilders$VirtualThreadBuilder:java/lang/ThreadBuilders:VirtualThreadBuilder:24;java/lang/Thread$ThreadNumbering:java/lang/Thread:ThreadNumbering:10;java/lang/Thread$UncaughtExceptionHandler:java/lang/Thread:UncaughtExceptionHandler:1545;java/lang/Thread$State:java/lang/Thread:State:16409;java/lang/Thread$Caches:java/lang/Thread:Caches:10;java/lang/Thread$1:::0;java/lang/Thread$Builder:java/lang/Thread:Builder:1545;java/lang/Thread$Builder$OfPlatform:java/lang/Thread$Builder:OfPlatform:1545;java/lang/Thread$Builder$OfVirtual:java/lang/Thread$Builder:OfVirtual:1545;java/lang/Thread$Constants$1:::0;java/lang/Thread$Caches$1:::0",
    all_supertypes    = "java/lang/Object;java/lang/Runnable;java/lang/Thread",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Thread {
    #[cfg_attr(any(), java_field(name = "eetop", descriptor = "J", access = "private", modifiers = "volatile", is_static = false))]
    pub eetop: JField<i64>,
    #[cfg_attr(any(), java_field(name = "tid", descriptor = "J", access = "private", modifiers = "final", is_static = false))]
    pub tid: JField<i64>,
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "volatile", is_static = false))]
    pub name: JField<String>,
    #[cfg_attr(any(), java_field(name = "interrupted", descriptor = "Z", access = "package", modifiers = "volatile", is_static = false))]
    pub interrupted: JField<bool>,
    #[cfg_attr(any(), java_field(name = "contextClassLoader", descriptor = "Ljava/lang/ClassLoader;", access = "private", modifiers = "volatile", is_static = false))]
    pub contextClassLoader: JField<Object>,
    #[cfg_attr(any(), java_field(name = "inheritedAccessControlContext", descriptor = "Ljava/security/AccessControlContext;", access = "private", modifiers = "", is_static = false))]
    pub inheritedAccessControlContext: JField<Object>,
    #[cfg_attr(any(), java_field(name = "holder", descriptor = "Ljava/lang/Thread$FieldHolder;", access = "private", modifiers = "final", is_static = false))]
    pub holder: JField<Thread_FieldHolder>,
    #[cfg_attr(any(), java_field(name = "threadLocals", descriptor = "Ljava/lang/ThreadLocal$ThreadLocalMap;", is_static = false))]
    pub threadLocals: JField<Object>,
    #[cfg_attr(any(), java_field(name = "inheritableThreadLocals", descriptor = "Ljava/lang/ThreadLocal$ThreadLocalMap;", is_static = false))]
    pub inheritableThreadLocals: JField<Object>,
    #[cfg_attr(any(), java_field(name = "scopedValueBindings", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "", is_static = false))]
    pub scopedValueBindings: JField<Object>,
    #[cfg_attr(any(), java_field(name = "interruptLock", descriptor = "Ljava/lang/Object;", access = "package", modifiers = "final", is_static = false))]
    pub interruptLock: JField<Object>,
    #[cfg_attr(any(), java_field(name = "parkBlocker", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "volatile", is_static = false))]
    pub parkBlocker: JField<Object>,
    #[cfg_attr(any(), java_field(name = "nioBlocker", descriptor = "Lsun/nio/ch/Interruptible;", access = "package", modifiers = "volatile", is_static = false))]
    pub nioBlocker: JField<Interruptible>,
    #[cfg_attr(any(), java_field(name = "cont", descriptor = "Ljdk/internal/vm/Continuation;", access = "private", modifiers = "", is_static = false))]
    pub cont: JField<Object>,
    #[cfg_attr(any(), java_field(name = "uncaughtExceptionHandler", descriptor = "Ljava/lang/Thread$UncaughtExceptionHandler;", access = "private", modifiers = "volatile", is_static = false))]
    pub uncaughtExceptionHandler: JField<Object>,
    #[cfg_attr(any(), java_field(name = "threadLocalRandomSeed", descriptor = "J", is_static = false))]
    pub threadLocalRandomSeed: JField<i64>,
    #[cfg_attr(any(), java_field(name = "threadLocalRandomProbe", descriptor = "I", is_static = false))]
    pub threadLocalRandomProbe: JField<i32>,
    #[cfg_attr(any(), java_field(name = "threadLocalRandomSecondarySeed", descriptor = "I", is_static = false))]
    pub threadLocalRandomSecondarySeed: JField<i32>,
    #[cfg_attr(any(), java_field(name = "container", descriptor = "Ljdk/internal/vm/ThreadContainer;", access = "private", modifiers = "", is_static = false))]
    pub container: JField<Object>,
    #[cfg_attr(any(), java_field(name = "headStackableScopes", descriptor = "Ljdk/internal/vm/StackableScope;", access = "private", modifiers = "volatile", is_static = false))]
    pub headStackableScopes: JField<Object>,
}

impl Thread {
    #[cfg_attr(any(), java_field(name = "NEW_THREAD_BINDINGS", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
    // static field: NEW_THREAD_BINDINGS:Ljava/lang/Object;
    pub fn NEW_THREAD_BINDINGS() -> Object {
        panic!("stub: java/lang/Thread.NEW_THREAD_BINDINGS:Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_field(name = "MIN_PRIORITY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
    // static field: MIN_PRIORITY:I
    pub fn MIN_PRIORITY() -> i32 {
        1
    }

    #[cfg_attr(any(), java_field(name = "NORM_PRIORITY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
    // static field: NORM_PRIORITY:I
    pub fn NORM_PRIORITY() -> i32 {
        5
    }

    #[cfg_attr(any(), java_field(name = "MAX_PRIORITY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "10"))]
    // static field: MAX_PRIORITY:I
    pub fn MAX_PRIORITY() -> i32 {
        10
    }

    #[cfg_attr(any(), java_field(name = "NO_INHERIT_THREAD_LOCALS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4"))]
    // static field: NO_INHERIT_THREAD_LOCALS:I
    pub fn NO_INHERIT_THREAD_LOCALS() -> i32 {
        4
    }

    #[cfg_attr(any(), java_field(name = "EMPTY_STACK_TRACE", descriptor = "[Ljava/lang/StackTraceElement;", access = "private", modifiers = "static final", is_static = true))]
    // static field: EMPTY_STACK_TRACE:[Ljava/lang/StackTraceElement;
    pub fn EMPTY_STACK_TRACE() -> Rc<RefCell<Vec<Object>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[cfg_attr(any(), java_field(name = "defaultUncaughtExceptionHandler", descriptor = "Ljava/lang/Thread$UncaughtExceptionHandler;", access = "private", modifiers = "static volatile", is_static = true))]
    // static field: defaultUncaughtExceptionHandler:Ljava/lang/Thread$UncaughtExceptionHandler;
    pub fn defaultUncaughtExceptionHandler() -> Object {
        panic!("stub: java/lang/Thread.defaultUncaughtExceptionHandler:Ljava/lang/Thread$UncaughtExceptionHandler;")
    }

    #[cfg_attr(any(), java_native(name = "registerNatives", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn registerNatives() -> Result<()> {
        panic!("native: java/lang/Thread.registerNatives:()V")
    }

    #[cfg_attr(any(), java_method(name = "scopedValueBindings", descriptor = "()Ljava/lang/Object;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn scopedValueBindings() -> Result<Object> {
        panic!("stub: java/lang/Thread.scopedValueBindings:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "setScopedValueBindings", descriptor = "(Ljava/lang/Object;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setScopedValueBindings(bindings: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.setScopedValueBindings:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_native(name = "findScopedValueBindings", descriptor = "()Ljava/lang/Object;", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn findScopedValueBindings() -> Result<Object> {
        panic!("native: java/lang/Thread.findScopedValueBindings:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "inheritScopedValueBindings", descriptor = "(Ljdk/internal/vm/ThreadContainer;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn inheritScopedValueBindings(&self, container: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.inheritScopedValueBindings:(Ljdk/internal/vm/ThreadContainer;)V")
    }

    #[cfg_attr(any(), java_method(name = "blockedOn", descriptor = "(Lsun/nio/ch/Interruptible;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn blockedOn(b: Interruptible) -> Result<()> {
        panic!("stub: java/lang/Thread.blockedOn:(Lsun/nio/ch/Interruptible;)V")
    }

    #[cfg_attr(any(), java_method(name = "getContinuation", descriptor = "()Ljdk/internal/vm/Continuation;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getContinuation(&self) -> Result<Object> {
        panic!("stub: java/lang/Thread.getContinuation:()Ljdk/internal/vm/Continuation;")
    }

    #[cfg_attr(any(), java_method(name = "setContinuation", descriptor = "(Ljdk/internal/vm/Continuation;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setContinuation(&self, cont: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.setContinuation:(Ljdk/internal/vm/Continuation;)V")
    }

    #[cfg_attr(any(), java_native(name = "currentCarrierThread", descriptor = "()Ljava/lang/Thread;", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn currentCarrierThread() -> Result<Thread> {
        panic!("native: java/lang/Thread.currentCarrierThread:()Ljava/lang/Thread;")
    }

    #[cfg_attr(any(), java_native(name = "currentThread", descriptor = "()Ljava/lang/Thread;", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn currentThread() -> Result<Thread> {
        panic!("native: java/lang/Thread.currentThread:()Ljava/lang/Thread;")
    }

    #[cfg_attr(any(), java_native(name = "setCurrentThread", descriptor = "(Ljava/lang/Thread;)V", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn setCurrentThread(&self, arg0: Thread) -> Result<()> {
        panic!("native: java/lang/Thread.setCurrentThread:(Ljava/lang/Thread;)V")
    }

    #[cfg_attr(any(), java_native(name = "scopedValueCache", descriptor = "()[Ljava/lang/Object;", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn scopedValueCache() -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Thread.scopedValueCache:()[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_native(name = "setScopedValueCache", descriptor = "([Ljava/lang/Object;)V", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn setScopedValueCache(arg0: Rc<RefCell<Vec<Object>>>) -> Result<()> {
        panic!("native: java/lang/Thread.setScopedValueCache:([Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_native(name = "ensureMaterializedForStackWalk", descriptor = "(Ljava/lang/Object;)V", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn ensureMaterializedForStackWalk(arg0: Object) -> Result<()> {
        panic!("native: java/lang/Thread.ensureMaterializedForStackWalk:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "yield", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn yield_() -> Result<()> {
        panic!("stub: java/lang/Thread.yield:()V")
    }

    #[cfg_attr(any(), java_native(name = "yield0", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn yield0() -> Result<()> {
        panic!("native: java/lang/Thread.yield0:()V")
    }

    #[cfg_attr(any(), java_method(name = "beforeSleep", descriptor = "(J)Ljdk/internal/event/ThreadSleepEvent;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn beforeSleep(nanos: i64) -> Result<Object> {
        panic!("stub: java/lang/Thread.beforeSleep:(J)Ljdk/internal/event/ThreadSleepEvent;")
    }

    #[cfg_attr(any(), java_method(name = "afterSleep", descriptor = "(Ljdk/internal/event/ThreadSleepEvent;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn afterSleep(event: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.afterSleep:(Ljdk/internal/event/ThreadSleepEvent;)V")
    }

    #[cfg_attr(any(), java_method(name = "sleep", descriptor = "(J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn sleep_l(millis: i64) -> Result<()> {
        panic!("stub: java/lang/Thread.sleep:(J)V")
    }

    #[cfg_attr(any(), java_native(name = "sleep0", descriptor = "(J)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn sleep0(arg0: i64) -> Result<()> {
        panic!("native: java/lang/Thread.sleep0:(J)V")
    }

    #[cfg_attr(any(), java_method(name = "sleep", descriptor = "(JI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn sleep_l_i(millis: i64, arg1: i32) -> Result<()> {
        panic!("stub: java/lang/Thread.sleep:(JI)V")
    }

    #[cfg_attr(any(), java_method(name = "sleep", descriptor = "(Ljava/time/Duration;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn sleep_durati(duration: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.sleep:(Ljava/time/Duration;)V")
    }

    #[cfg_attr(any(), java_method(name = "onSpinWait", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn onSpinWait() -> Result<()> {
        panic!("stub: java/lang/Thread.onSpinWait:()V")
    }

    #[cfg_attr(any(), java_method(name = "contextClassLoader", descriptor = "(Ljava/lang/Thread;)Ljava/lang/ClassLoader;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn contextClassLoader(parent: Thread) -> Result<Object> {
        panic!("stub: java/lang/Thread.contextClassLoader:(Ljava/lang/Thread;)Ljava/lang/ClassLoader;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/ThreadGroup;Ljava/lang/String;ILjava/lang/Runnable;JLjava/security/AccessControlContext;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_thread_str_i_runnab_l_access(g: Object, name: String, characteristics: i32, task: Object, stackSize: i64, arg5: Object) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/ThreadGroup;Ljava/lang/String;ILjava/lang/Runnable;JLjava/security/AccessControlContext;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str_i_z(name: String, characteristics: i32, bound: bool) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/String;IZ)V")
    }

    #[cfg_attr(any(), java_method(name = "ofPlatform", descriptor = "()Ljava/lang/Thread$Builder$OfPlatform;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ofPlatform() -> Result<Object> {
        panic!("stub: java/lang/Thread.ofPlatform:()Ljava/lang/Thread$Builder$OfPlatform;")
    }

    #[cfg_attr(any(), java_method(name = "ofVirtual", descriptor = "()Ljava/lang/Thread$Builder$OfVirtual;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ofVirtual() -> Result<Object> {
        panic!("stub: java/lang/Thread.ofVirtual:()Ljava/lang/Thread$Builder$OfVirtual;")
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/CloneNotSupportedException"))]
    pub fn clone(&self) -> Result<Object> {
        panic!("stub: java/lang/Thread.clone:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "genThreadName", descriptor = "()Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn genThreadName() -> Result<String> {
        panic!("stub: java/lang/Thread.genThreadName:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "checkName", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkName(name: String) -> Result<String> {
        panic!("stub: java/lang/Thread.checkName:(Ljava/lang/String;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Runnable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_runnab(task: Object) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/Runnable;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Runnable;Ljava/security/AccessControlContext;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_runnab_access(task: Object, acc: Object) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/Runnable;Ljava/security/AccessControlContext;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_thread_runnab(group: Object, task: Object) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str(name: String) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/ThreadGroup;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_thread_str(group: Object, name: String) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/ThreadGroup;Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Runnable;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_runnab_str(task: Object, name: String) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/Runnable;Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_thread_runnab_str(group: Object, task: Object, name: String) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;Ljava/lang/String;J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_thread_runnab_str_l(group: Object, task: Object, name: String, stackSize: i64) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;Ljava/lang/String;J)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;Ljava/lang/String;JZ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_thread_runnab_str_l_z(group: Object, task: Object, name: String, stackSize: i64, arg4: bool) -> Result<Self> {
        panic!("stub: java/lang/Thread.<init>:(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;Ljava/lang/String;JZ)V")
    }

    #[cfg_attr(any(), java_method(name = "startVirtualThread", descriptor = "(Ljava/lang/Runnable;)Ljava/lang/Thread;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn startVirtualThread(task: Object) -> Result<Thread> {
        panic!("stub: java/lang/Thread.startVirtualThread:(Ljava/lang/Runnable;)Ljava/lang/Thread;")
    }

    #[cfg_attr(any(), java_method(name = "isVirtual", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isVirtual(&self) -> Result<bool> {
        let this = self;
        Ok(false)
    }

    #[cfg_attr(any(), java_method(name = "start", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn start(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.start:()V")
    }

    #[cfg_attr(any(), java_method(name = "start", descriptor = "(Ljdk/internal/vm/ThreadContainer;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn start_thread(&self, container: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.start:(Ljdk/internal/vm/ThreadContainer;)V")
    }

    #[cfg_attr(any(), java_native(name = "start0", descriptor = "()V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn start0(&self) -> Result<()> {
        panic!("native: java/lang/Thread.start0:()V")
    }

    #[cfg_attr(any(), java_method(name = "run", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn run(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.run:()V")
    }

    #[cfg_attr(any(), java_method(name = "runWith", descriptor = "(Ljava/lang/Object;Ljava/lang/Runnable;)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn runWith(&self, bindings: Object, op: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.runWith:(Ljava/lang/Object;Ljava/lang/Runnable;)V")
    }

    #[cfg_attr(any(), java_method(name = "clearReferences", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clearReferences(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.clearReferences:()V")
    }

    #[cfg_attr(any(), java_method(name = "exit", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn exit(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.exit:()V")
    }

    #[cfg_attr(any(), java_method(name = "stop", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn stop(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.stop:()V")
    }

    #[cfg_attr(any(), java_method(name = "interrupted", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn interrupted() -> Result<bool> {
        panic!("stub: java/lang/Thread.interrupted:()Z")
    }

    #[cfg_attr(any(), java_method(name = "isInterrupted", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isInterrupted(&self) -> Result<bool> {
        panic!("stub: java/lang/Thread.isInterrupted:()Z")
    }

    #[cfg_attr(any(), java_method(name = "setInterrupt", descriptor = "()V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setInterrupt(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.setInterrupt:()V")
    }

    #[cfg_attr(any(), java_method(name = "clearInterrupt", descriptor = "()V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clearInterrupt(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.clearInterrupt:()V")
    }

    #[cfg_attr(any(), java_method(name = "getAndClearInterrupt", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getAndClearInterrupt(&self) -> Result<bool> {
        panic!("stub: java/lang/Thread.getAndClearInterrupt:()Z")
    }

    #[cfg_attr(any(), java_method(name = "isAlive", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isAlive(&self) -> Result<bool> {
        panic!("stub: java/lang/Thread.isAlive:()Z")
    }

    #[cfg_attr(any(), java_method(name = "alive", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn alive(&self) -> Result<bool> {
        panic!("stub: java/lang/Thread.alive:()Z")
    }

    #[cfg_attr(any(), java_method(name = "suspend", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn suspend(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.suspend:()V")
    }

    #[cfg_attr(any(), java_method(name = "resume", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn resume(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.resume:()V")
    }

    #[cfg_attr(any(), java_method(name = "setPriority", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setPriority(&self, newPriority: i32) -> Result<()> {
        panic!("stub: java/lang/Thread.setPriority:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "priority", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn priority(&self, newPriority: i32) -> Result<()> {
        panic!("stub: java/lang/Thread.priority:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "getPriority", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getPriority(&self) -> Result<i32> {
        panic!("stub: java/lang/Thread.getPriority:()I")
    }

    #[cfg_attr(any(), java_method(name = "setName", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "final synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setName(&self, name: String) -> Result<()> {
        panic!("stub: java/lang/Thread.setName:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getName(&self) -> Result<String> {
        panic!("stub: java/lang/Thread.getName:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "activeCount", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn activeCount() -> Result<i32> {
        panic!("stub: java/lang/Thread.activeCount:()I")
    }

    #[cfg_attr(any(), java_method(name = "enumerate", descriptor = "([Ljava/lang/Thread;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn enumerate(tarray: Rc<RefCell<Vec<Thread>>>) -> Result<i32> {
        panic!("stub: java/lang/Thread.enumerate:([Ljava/lang/Thread;)I")
    }

    #[cfg_attr(any(), java_method(name = "countStackFrames", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn countStackFrames(&self) -> Result<i32> {
        panic!("stub: java/lang/Thread.countStackFrames:()I")
    }

    #[cfg_attr(any(), java_method(name = "join", descriptor = "(J)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn join_l(&self, millis: i64) -> Result<()> {
        panic!("stub: java/lang/Thread.join:(J)V")
    }

    #[cfg_attr(any(), java_method(name = "join", descriptor = "(JI)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn join_l_i(&self, millis: i64, arg1: i32) -> Result<()> {
        panic!("stub: java/lang/Thread.join:(JI)V")
    }

    #[cfg_attr(any(), java_method(name = "join", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn join(&self) -> Result<()> {
        panic!("stub: java/lang/Thread.join:()V")
    }

    #[cfg_attr(any(), java_method(name = "join", descriptor = "(Ljava/time/Duration;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException"))]
    pub fn join_durati(&self, duration: Object) -> Result<bool> {
        panic!("stub: java/lang/Thread.join:(Ljava/time/Duration;)Z")
    }

    #[cfg_attr(any(), java_method(name = "dumpStack", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn dumpStack() -> Result<()> {
        panic!("stub: java/lang/Thread.dumpStack:()V")
    }

    #[cfg_attr(any(), java_method(name = "setDaemon", descriptor = "(Z)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setDaemon(&self, on: bool) -> Result<()> {
        panic!("stub: java/lang/Thread.setDaemon:(Z)V")
    }

    #[cfg_attr(any(), java_method(name = "daemon", descriptor = "(Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn daemon(&self, on: bool) -> Result<()> {
        panic!("stub: java/lang/Thread.daemon:(Z)V")
    }

    #[cfg_attr(any(), java_method(name = "isDaemon", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isDaemon(&self) -> Result<bool> {
        panic!("stub: java/lang/Thread.isDaemon:()Z")
    }

    #[cfg_attr(any(), java_method(name = "checkAccess", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn checkAccess(&self) -> Result<()> {
        let this = self;
        let _t0: SecurityManager = System::getSecurityManager()?;
        let mut security: SecurityManager = _t0;
        if !_is_jnull(&security) {
            security.checkAccess_thread(Clone::clone(&this))?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Thread.toString:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "getContextClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getContextClassLoader(&self) -> Result<Object> {
        panic!("stub: java/lang/Thread.getContextClassLoader:()Ljava/lang/ClassLoader;")
    }

    #[cfg_attr(any(), java_method(name = "setContextClassLoader", descriptor = "(Ljava/lang/ClassLoader;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setContextClassLoader(&self, cl: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.setContextClassLoader:(Ljava/lang/ClassLoader;)V")
    }

    #[cfg_attr(any(), java_native(name = "holdsLock", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn holdsLock(arg0: Object) -> Result<bool> {
        panic!("native: java/lang/Thread.holdsLock:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "getStackTrace", descriptor = "()[Ljava/lang/StackTraceElement;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getStackTrace(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Thread.getStackTrace:()[Ljava/lang/StackTraceElement;")
    }

    #[cfg_attr(any(), java_method(name = "asyncGetStackTrace", descriptor = "()[Ljava/lang/StackTraceElement;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn asyncGetStackTrace(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Thread.asyncGetStackTrace:()[Ljava/lang/StackTraceElement;")
    }

    #[cfg_attr(any(), java_native(name = "getStackTrace0", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getStackTrace0(&self) -> Result<Object> {
        panic!("native: java/lang/Thread.getStackTrace0:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "getAllStackTraces", descriptor = "()Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/Thread;[Ljava/lang/StackTraceElement;>;"))]
    pub fn getAllStackTraces() -> Result<Object> {
        panic!("stub: java/lang/Thread.getAllStackTraces:()Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "isCCLOverridden", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z"))]
    pub fn isCCLOverridden(cl: Object) -> Result<bool> {
        panic!("stub: java/lang/Thread.isCCLOverridden:(Ljava/lang/Class;)Z")
    }

    #[cfg_attr(any(), java_method(name = "auditSubclass", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z"))]
    pub fn auditSubclass(subcl: Object) -> Result<bool> {
        panic!("stub: java/lang/Thread.auditSubclass:(Ljava/lang/Class;)Z")
    }

    #[cfg_attr(any(), java_method(name = "getAllThreads", descriptor = "()[Ljava/lang/Thread;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getAllThreads() -> Result<Rc<RefCell<Vec<Thread>>>> {
        panic!("stub: java/lang/Thread.getAllThreads:()[Ljava/lang/Thread;")
    }

    #[cfg_attr(any(), java_native(name = "dumpThreads", descriptor = "([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn dumpThreads(arg0: Rc<RefCell<Vec<Thread>>>) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<Object>>>>>>> {
        panic!("native: java/lang/Thread.dumpThreads:([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;")
    }

    #[cfg_attr(any(), java_native(name = "getThreads", descriptor = "()[Ljava/lang/Thread;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getThreads() -> Result<Rc<RefCell<Vec<Thread>>>> {
        panic!("native: java/lang/Thread.getThreads:()[Ljava/lang/Thread;")
    }

    #[cfg_attr(any(), java_method(name = "getId", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn getId(&self) -> Result<i64> {
        panic!("stub: java/lang/Thread.getId:()J")
    }

    #[cfg_attr(any(), java_method(name = "threadId", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn threadId(&self) -> Result<i64> {
        panic!("stub: java/lang/Thread.threadId:()J")
    }

    #[cfg_attr(any(), java_method(name = "getState", descriptor = "()Ljava/lang/Thread$State;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getState(&self) -> Result<Thread_State> {
        panic!("stub: java/lang/Thread.getState:()Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_method(name = "threadState", descriptor = "()Ljava/lang/Thread$State;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn threadState(&self) -> Result<Thread_State> {
        let this = self;
        let _t0: Thread_State = VM::toThreadState(this.holder.get().threadStatus.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "setDefaultUncaughtExceptionHandler", descriptor = "(Ljava/lang/Thread$UncaughtExceptionHandler;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setDefaultUncaughtExceptionHandler(ueh: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.setDefaultUncaughtExceptionHandler:(Ljava/lang/Thread$UncaughtExceptionHandler;)V")
    }

    #[cfg_attr(any(), java_method(name = "getDefaultUncaughtExceptionHandler", descriptor = "()Ljava/lang/Thread$UncaughtExceptionHandler;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getDefaultUncaughtExceptionHandler() -> Result<Object> {
        panic!("stub: java/lang/Thread.getDefaultUncaughtExceptionHandler:()Ljava/lang/Thread$UncaughtExceptionHandler;")
    }

    #[cfg_attr(any(), java_method(name = "getUncaughtExceptionHandler", descriptor = "()Ljava/lang/Thread$UncaughtExceptionHandler;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getUncaughtExceptionHandler(&self) -> Result<Object> {
        panic!("stub: java/lang/Thread.getUncaughtExceptionHandler:()Ljava/lang/Thread$UncaughtExceptionHandler;")
    }

    #[cfg_attr(any(), java_method(name = "setUncaughtExceptionHandler", descriptor = "(Ljava/lang/Thread$UncaughtExceptionHandler;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setUncaughtExceptionHandler(&self, ueh: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.setUncaughtExceptionHandler:(Ljava/lang/Thread$UncaughtExceptionHandler;)V")
    }

    #[cfg_attr(any(), java_method(name = "uncaughtExceptionHandler", descriptor = "(Ljava/lang/Thread$UncaughtExceptionHandler;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn uncaughtExceptionHandler(&self, ueh: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.uncaughtExceptionHandler:(Ljava/lang/Thread$UncaughtExceptionHandler;)V")
    }

    #[cfg_attr(any(), java_method(name = "dispatchUncaughtException", descriptor = "(Ljava/lang/Throwable;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn dispatchUncaughtException(&self, e: Throwable) -> Result<()> {
        panic!("stub: java/lang/Thread.dispatchUncaughtException:(Ljava/lang/Throwable;)V")
    }

    #[cfg_attr(any(), java_method(name = "virtualThreadGroup", descriptor = "()Ljava/lang/ThreadGroup;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn virtualThreadGroup() -> Result<Object> {
        Ok(Thread_Constants::VTHREAD_GROUP())
    }

    #[cfg_attr(any(), java_method(name = "threadContainer", descriptor = "()Ljdk/internal/vm/ThreadContainer;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn threadContainer(&self) -> Result<Object> {
        panic!("stub: java/lang/Thread.threadContainer:()Ljdk/internal/vm/ThreadContainer;")
    }

    #[cfg_attr(any(), java_method(name = "setThreadContainer", descriptor = "(Ljdk/internal/vm/ThreadContainer;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setThreadContainer(&self, container: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.setThreadContainer:(Ljdk/internal/vm/ThreadContainer;)V")
    }

    #[cfg_attr(any(), java_method(name = "headStackableScopes", descriptor = "()Ljdk/internal/vm/StackableScope;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn headStackableScopes(&self) -> Result<Object> {
        panic!("stub: java/lang/Thread.headStackableScopes:()Ljdk/internal/vm/StackableScope;")
    }

    #[cfg_attr(any(), java_method(name = "setHeadStackableScope", descriptor = "(Ljdk/internal/vm/StackableScope;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setHeadStackableScope(scope: Object) -> Result<()> {
        panic!("stub: java/lang/Thread.setHeadStackableScope:(Ljdk/internal/vm/StackableScope;)V")
    }

    #[cfg_attr(any(), java_native(name = "setPriority0", descriptor = "(I)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn setPriority0(&self, arg0: i32) -> Result<()> {
        panic!("native: java/lang/Thread.setPriority0:(I)V")
    }

    #[cfg_attr(any(), java_native(name = "interrupt0", descriptor = "()V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn interrupt0(&self) -> Result<()> {
        panic!("native: java/lang/Thread.interrupt0:()V")
    }

    #[cfg_attr(any(), java_native(name = "clearInterruptEvent", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn clearInterruptEvent() -> Result<()> {
        panic!("native: java/lang/Thread.clearInterruptEvent:()V")
    }

    #[cfg_attr(any(), java_native(name = "setNativeName", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn setNativeName(&self, arg0: String) -> Result<()> {
        panic!("native: java/lang/Thread.setNativeName:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_native(name = "getNextThreadIdOffset", descriptor = "()J", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getNextThreadIdOffset() -> Result<i64> {
        panic!("native: java/lang/Thread.getNextThreadIdOffset:()J")
    }
}
