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
use crate::jdk::internal::misc::Unsafe;
use crate::jdk::internal::vm::Continuation;

impl From<VirtualThread> for BaseVirtualThread {
    fn from(v: VirtualThread) -> BaseVirtualThread { v.__into_super() }
}

impl From<VirtualThread> for Thread {
    fn from(v: VirtualThread) -> Thread { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/VirtualThread"]
    #[super_class       = "java/lang/BaseVirtualThread"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "VirtualThread.java"]
    #[inner_classes     = "java/lang/VirtualThread$VThreadContinuation:java/lang/VirtualThread:VThreadContinuation:10;java/lang/Thread$State:java/lang/Thread:State:16409;java/util/concurrent/ForkJoinPool$ForkJoinWorkerThreadFactory:java/util/concurrent/ForkJoinPool:ForkJoinWorkerThreadFactory:1545;java/lang/Thread$UncaughtExceptionHandler:java/lang/Thread:UncaughtExceptionHandler:1545;java/lang/VirtualThread$VThreadContinuation$1:::0;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "BaseVirtualThread"]
    #[superclass_fields(eetop: i64, tid: i64, name: String, interrupted: bool, contextClassLoader: ClassLoader, inheritedAccessControlContext: Object, holder: Thread_FieldHolder, threadLocals: Object, inheritableThreadLocals: Object, scopedValueBindings: Object, interruptLock: Object, parkBlocker: Object, nioBlocker: Object, cont: Continuation, uncaughtExceptionHandler: Object, threadLocalRandomSeed: i64, threadLocalRandomProbe: i32, threadLocalRandomSecondarySeed: i32, container: Object, headStackableScopes: Object)]
    #[all_supertypes    = "java/lang/BaseVirtualThread;java/lang/Object;java/lang/Runnable;java/lang/Thread;java/lang/VirtualThread"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct VirtualThread {
        #[cfg_attr(any(), java_field(name = "scheduler", descriptor = "Ljava/util/concurrent/Executor;", access = "private", modifiers = "final", is_static = false))]
        pub scheduler: Object,
        #[cfg_attr(any(), java_field(name = "cont", descriptor = "Ljdk/internal/vm/Continuation;", access = "private", modifiers = "final", is_static = false))]
        pub cont: Continuation,
        #[cfg_attr(any(), java_field(name = "runContinuation", descriptor = "Ljava/lang/Runnable;", access = "private", modifiers = "final", is_static = false))]
        pub runContinuation: Object,
        #[cfg_attr(any(), java_field(name = "state", descriptor = "I", access = "private", modifiers = "volatile", is_static = false))]
        pub state: i32,
        #[cfg_attr(any(), java_field(name = "parkPermit", descriptor = "Z", access = "private", modifiers = "volatile", is_static = false))]
        pub parkPermit: bool,
        #[cfg_attr(any(), java_field(name = "carrierThread", descriptor = "Ljava/lang/Thread;", access = "private", modifiers = "volatile", is_static = false))]
        pub carrierThread: Thread,
        #[cfg_attr(any(), java_field(name = "termination", descriptor = "Ljava/util/concurrent/CountDownLatch;", access = "private", modifiers = "volatile", is_static = false))]
        pub termination: Object,
    }

    impl VirtualThread {
        #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: U:Ljdk/internal/misc/Unsafe;
        pub fn U() -> Unsafe {
            panic!("stub: java/lang/VirtualThread.U:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "VTHREAD_SCOPE", descriptor = "Ljdk/internal/vm/ContinuationScope;", access = "private", modifiers = "static final", is_static = true))]
        // static field: VTHREAD_SCOPE:Ljdk/internal/vm/ContinuationScope;
        pub fn VTHREAD_SCOPE() -> Object {
            panic!("stub: java/lang/VirtualThread.VTHREAD_SCOPE:Ljdk/internal/vm/ContinuationScope;")
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_SCHEDULER", descriptor = "Ljava/util/concurrent/ForkJoinPool;", access = "private", modifiers = "static final", is_static = true))]
        // static field: DEFAULT_SCHEDULER:Ljava/util/concurrent/ForkJoinPool;
        pub fn DEFAULT_SCHEDULER() -> Object {
            panic!("stub: java/lang/VirtualThread.DEFAULT_SCHEDULER:Ljava/util/concurrent/ForkJoinPool;")
        }

        #[cfg_attr(any(), java_field(name = "UNPARKER", descriptor = "Ljava/util/concurrent/ScheduledExecutorService;", access = "private", modifiers = "static final", is_static = true))]
        // static field: UNPARKER:Ljava/util/concurrent/ScheduledExecutorService;
        pub fn UNPARKER() -> Object {
            panic!("stub: java/lang/VirtualThread.UNPARKER:Ljava/util/concurrent/ScheduledExecutorService;")
        }

        #[cfg_attr(any(), java_field(name = "TRACE_PINNING_MODE", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: TRACE_PINNING_MODE:I
        pub fn TRACE_PINNING_MODE() -> i32 {
            panic!("stub: java/lang/VirtualThread.TRACE_PINNING_MODE:I")
        }

        #[cfg_attr(any(), java_field(name = "STATE", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: STATE:J
        pub fn STATE() -> i64 {
            panic!("stub: java/lang/VirtualThread.STATE:J")
        }

        #[cfg_attr(any(), java_field(name = "PARK_PERMIT", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: PARK_PERMIT:J
        pub fn PARK_PERMIT() -> i64 {
            panic!("stub: java/lang/VirtualThread.PARK_PERMIT:J")
        }

        #[cfg_attr(any(), java_field(name = "CARRIER_THREAD", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: CARRIER_THREAD:J
        pub fn CARRIER_THREAD() -> i64 {
            panic!("stub: java/lang/VirtualThread.CARRIER_THREAD:J")
        }

        #[cfg_attr(any(), java_field(name = "TERMINATION", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: TERMINATION:J
        pub fn TERMINATION() -> i64 {
            panic!("stub: java/lang/VirtualThread.TERMINATION:J")
        }

        #[cfg_attr(any(), java_field(name = "NEW", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: NEW:I
        pub fn NEW() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "STARTED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: STARTED:I
        pub fn STARTED() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "RUNNING", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: RUNNING:I
        pub fn RUNNING() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "PARKING", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: PARKING:I
        pub fn PARKING() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "PARKED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: PARKED:I
        pub fn PARKED() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "PINNED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: PINNED:I
        pub fn PINNED() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "TIMED_PARKING", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: TIMED_PARKING:I
        pub fn TIMED_PARKING() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "TIMED_PARKED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: TIMED_PARKED:I
        pub fn TIMED_PARKED() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "TIMED_PINNED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: TIMED_PINNED:I
        pub fn TIMED_PINNED() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "UNPARKED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: UNPARKED:I
        pub fn UNPARKED() -> i32 {
            9
        }

        #[cfg_attr(any(), java_field(name = "YIELDING", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: YIELDING:I
        pub fn YIELDING() -> i32 {
            10
        }

        #[cfg_attr(any(), java_field(name = "YIELDED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: YIELDED:I
        pub fn YIELDED() -> i32 {
            11
        }

        #[cfg_attr(any(), java_field(name = "TERMINATED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "99"))]
        // static field: TERMINATED:I
        pub fn TERMINATED() -> i32 {
            99
        }

        #[cfg_attr(any(), java_field(name = "SUSPENDED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "256"))]
        // static field: SUSPENDED:I
        pub fn SUSPENDED() -> i32 {
            256
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "continuationScope", descriptor = "()Ljdk/internal/vm/ContinuationScope;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn continuationScope() -> Result<Object> {
            panic!("stub: java/lang/VirtualThread.continuationScope:()Ljdk/internal/vm/ContinuationScope;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/concurrent/Executor;Ljava/lang/String;ILjava/lang/Runnable;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(scheduler: Object, name: String, characteristics: i32, task: Object) -> Result<Self> {
            panic!("stub: java/lang/VirtualThread.<init>:(Ljava/util/concurrent/Executor;Ljava/lang/String;ILjava/lang/Runnable;)V")
        }

        #[java_method(name = "runContinuation", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn runContinuation(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.runContinuation:()V")
        }

        #[java_method(name = "submitRunContinuation", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn submitRunContinuation(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.submitRunContinuation:()V")
        }

        #[java_method(name = "lazySubmitRunContinuation", descriptor = "(Ljava/util/concurrent/ForkJoinPool;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lazySubmitRunContinuation(&self, pool: Object) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.lazySubmitRunContinuation:(Ljava/util/concurrent/ForkJoinPool;)V")
        }

        #[java_method(name = "externalSubmitRunContinuation", descriptor = "(Ljava/util/concurrent/ForkJoinPool;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn externalSubmitRunContinuation(&self, pool: Object) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.externalSubmitRunContinuation:(Ljava/util/concurrent/ForkJoinPool;)V")
        }

        #[java_method(name = "submitFailed", descriptor = "(Ljava/util/concurrent/RejectedExecutionException;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn submitFailed(&self, ree: Object) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.submitFailed:(Ljava/util/concurrent/RejectedExecutionException;)V")
        }

        #[java_method(name = "run", descriptor = "(Ljava/lang/Runnable;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn run_runnab(&self, task: Object) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.run:(Ljava/lang/Runnable;)V")
        }

        #[java_method(name = "mount", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mount(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.mount:()V")
        }

        #[java_method(name = "unmount", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unmount(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.unmount:()V")
        }

        #[java_method(name = "switchToCarrierThread", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn switchToCarrierThread(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.switchToCarrierThread:()V")
        }

        #[java_method(name = "switchToVirtualThread", descriptor = "(Ljava/lang/VirtualThread;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn switchToVirtualThread(&self, vthread: VirtualThread) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.switchToVirtualThread:(Ljava/lang/VirtualThread;)V")
        }

        #[java_method(name = "executeOnCarrierThread", descriptor = "(Ljava/util/concurrent/Callable;)Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/Exception", generic_signature = "<V:Ljava/lang/Object;>(Ljava/util/concurrent/Callable<TV;>;)TV;")]
        pub fn executeOnCarrierThread(&self, task: Object) -> Result<Object> {
            panic!("stub: java/lang/VirtualThread.executeOnCarrierThread:(Ljava/util/concurrent/Callable;)Ljava/lang/Object;")
        }

        #[java_method(name = "yieldContinuation", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn yieldContinuation(&self) -> Result<bool> {
            panic!("stub: java/lang/VirtualThread.yieldContinuation:()Z")
        }

        #[java_method(name = "afterYield", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn afterYield(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.afterYield:()V")
        }

        #[java_method(name = "afterDone", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn afterDone(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.afterDone:()V")
        }

        #[java_method(name = "afterDone", descriptor = "(Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn afterDone_z(&self, notifyContainer: bool) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.afterDone:(Z)V")
        }

        #[java_method(name = "start", descriptor = "(Ljdk/internal/vm/ThreadContainer;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn start_thread(&self, container: Object) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.start:(Ljdk/internal/vm/ThreadContainer;)V")
        }

        #[java_method(name = "start", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn start(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.start:()V")
        }

        #[java_method(name = "run", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn run(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.run:()V")
        }

        #[java_method(name = "park", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn park(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.park:()V")
        }

        #[java_method(name = "parkNanos", descriptor = "(J)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parkNanos(&self, nanos: i64) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.parkNanos:(J)V")
        }

        #[java_method(name = "parkOnCarrierThread", descriptor = "(ZJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parkOnCarrierThread(&self, timed: bool, nanos: i64) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.parkOnCarrierThread:(ZJ)V")
        }

        #[java_method(name = "scheduleUnpark", descriptor = "(J)Ljava/util/concurrent/Future;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(J)Ljava/util/concurrent/Future<*>;")]
        pub fn scheduleUnpark(&self, nanos: i64) -> Result<Object> {
            panic!("stub: java/lang/VirtualThread.scheduleUnpark:(J)Ljava/util/concurrent/Future;")
        }

        #[java_method(name = "cancel", descriptor = "(Ljava/util/concurrent/Future;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/Future<*>;)V")]
        pub fn cancel(&self, future: Object) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.cancel:(Ljava/util/concurrent/Future;)V")
        }

        #[java_method(name = "unpark", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unpark(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.unpark:()V")
        }

        #[java_method(name = "tryYield", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryYield(&self) -> Result<()> {
            let this = self;
            let _t0: Thread = Thread::currentThread()?;
            if Object::from_any(_t0.clone()) != Object::from_any(this.clone()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.setState(10i32)?;
            let mut yielded: i32 = 0i32;
            let _t1 = this.yieldContinuation()?;
            yielded = (_t1) as i32;
            let _t2: Thread = Thread::currentThread()?;
            let _t3 = this.state()?;
            if yielded != (_t3 == 2i32) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t4 = this.state()?;
            if _t4 != 10i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.setState(2i32)?;
            Ok(())
        }

        #[java_method(name = "sleepNanos", descriptor = "(J)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException")]
        pub fn sleepNanos(&self, nanos: i64) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.sleepNanos:(J)V")
        }

        #[java_method(name = "joinNanos", descriptor = "(J)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException")]
        pub fn joinNanos(&self, nanos: i64) -> Result<bool> {
            panic!("stub: java/lang/VirtualThread.joinNanos:(J)Z")
        }

        #[java_method(name = "interrupt", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn interrupt(&self) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.interrupt:()V")
        }

        #[java_method(name = "isInterrupted", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isInterrupted(&self) -> Result<bool> {
            panic!("stub: java/lang/VirtualThread.isInterrupted:()Z")
        }

        #[java_method(name = "getAndClearInterrupt", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndClearInterrupt(&self) -> Result<bool> {
            panic!("stub: java/lang/VirtualThread.getAndClearInterrupt:()Z")
        }

        #[java_method(name = "threadState", descriptor = "()Ljava/lang/Thread$State;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn threadState(&self) -> Result<Thread_State> {
            panic!("stub: java/lang/VirtualThread.threadState:()Ljava/lang/Thread$State;")
        }

        #[java_method(name = "alive", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn alive(&self) -> Result<bool> {
            panic!("stub: java/lang/VirtualThread.alive:()Z")
        }

        #[java_method(name = "isTerminated", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isTerminated(&self) -> Result<bool> {
            panic!("stub: java/lang/VirtualThread.isTerminated:()Z")
        }

        #[java_method(name = "asyncGetStackTrace", descriptor = "()[Ljava/lang/StackTraceElement;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asyncGetStackTrace(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/VirtualThread.asyncGetStackTrace:()[Ljava/lang/StackTraceElement;")
        }

        #[java_method(name = "tryGetStackTrace", descriptor = "()[Ljava/lang/StackTraceElement;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryGetStackTrace(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/VirtualThread.tryGetStackTrace:()[Ljava/lang/StackTraceElement;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/VirtualThread.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "getTermination", descriptor = "()Ljava/util/concurrent/CountDownLatch;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTermination(&self) -> Result<Object> {
            panic!("stub: java/lang/VirtualThread.getTermination:()Ljava/util/concurrent/CountDownLatch;")
        }

        #[java_method(name = "carrierThreadAccessLock", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn carrierThreadAccessLock(&self) -> Result<Object> {
            panic!("stub: java/lang/VirtualThread.carrierThreadAccessLock:()Ljava/lang/Object;")
        }

        #[java_method(name = "state", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn state(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_state())
        }

        #[java_method(name = "setState", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setState(&self, mut newValue: i32) -> Result<()> {
            let this = self;
            this.__set_state(newValue);
            Ok(())
        }

        #[java_method(name = "compareAndSetState", descriptor = "(II)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetState(&self, expectedValue: i32, newValue: i32) -> Result<bool> {
            panic!("stub: java/lang/VirtualThread.compareAndSetState:(II)Z")
        }

        #[java_method(name = "setParkPermit", descriptor = "(Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setParkPermit(&self, newValue: bool) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.setParkPermit:(Z)V")
        }

        #[java_method(name = "getAndSetParkPermit", descriptor = "(Z)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetParkPermit(&self, newValue: bool) -> Result<bool> {
            panic!("stub: java/lang/VirtualThread.getAndSetParkPermit:(Z)Z")
        }

        #[java_method(name = "setCarrierThread", descriptor = "(Ljava/lang/Thread;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setCarrierThread(&self, carrier: Thread) -> Result<()> {
            panic!("stub: java/lang/VirtualThread.setCarrierThread:(Ljava/lang/Thread;)V")
        }

        #[native]
        #[java_native(name = "notifyJvmtiStart", descriptor = "()V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn notifyJvmtiStart(&self) -> Result<()> {
            panic!("native: java/lang/VirtualThread.notifyJvmtiStart:()V")
        }

        #[native]
        #[java_native(name = "notifyJvmtiEnd", descriptor = "()V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn notifyJvmtiEnd(&self) -> Result<()> {
            panic!("native: java/lang/VirtualThread.notifyJvmtiEnd:()V")
        }

        #[native]
        #[java_native(name = "notifyJvmtiMount", descriptor = "(Z)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn notifyJvmtiMount(&self, arg0: bool) -> Result<()> {
            panic!("native: java/lang/VirtualThread.notifyJvmtiMount:(Z)V")
        }

        #[native]
        #[java_native(name = "notifyJvmtiUnmount", descriptor = "(Z)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn notifyJvmtiUnmount(&self, arg0: bool) -> Result<()> {
            panic!("native: java/lang/VirtualThread.notifyJvmtiUnmount:(Z)V")
        }

        #[native]
        #[java_native(name = "notifyJvmtiHideFrames", descriptor = "(Z)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn notifyJvmtiHideFrames(&self, arg0: bool) -> Result<()> {
            panic!("native: java/lang/VirtualThread.notifyJvmtiHideFrames:(Z)V")
        }

        #[native]
        #[java_native(name = "registerNatives", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn registerNatives() -> Result<()> {
            panic!("native: java/lang/VirtualThread.registerNatives:()V")
        }

        #[java_method(name = "createDefaultScheduler", descriptor = "()Ljava/util/concurrent/ForkJoinPool;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createDefaultScheduler() -> Result<Object> {
            panic!("stub: java/lang/VirtualThread.createDefaultScheduler:()Ljava/util/concurrent/ForkJoinPool;")
        }

        #[java_method(name = "createDelayedTaskScheduler", descriptor = "()Ljava/util/concurrent/ScheduledExecutorService;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createDelayedTaskScheduler() -> Result<Object> {
            panic!("stub: java/lang/VirtualThread.createDelayedTaskScheduler:()Ljava/util/concurrent/ScheduledExecutorService;")
        }

        #[java_method(name = "tracePinningMode", descriptor = "()I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tracePinningMode() -> Result<i32> {
            panic!("stub: java/lang/VirtualThread.tracePinningMode:()I")
        }
    }
}
