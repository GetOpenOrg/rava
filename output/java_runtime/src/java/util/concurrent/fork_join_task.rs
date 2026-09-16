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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/ForkJoinTask"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/concurrent/Future,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<V:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/concurrent/Future<TV;>;Ljava/io/Serializable;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ForkJoinTask.java"]
    #[inner_classes     = "java/util/concurrent/ForkJoinTask$Aux:java/util/concurrent/ForkJoinTask:Aux:24;java/util/concurrent/ForkJoinPool$WorkQueue:java/util/concurrent/ForkJoinPool:WorkQueue:24;java/util/concurrent/Future$State:java/util/concurrent/Future:State:16409;java/util/concurrent/ForkJoinTask$AdaptedRunnableAction:java/util/concurrent/ForkJoinTask:AdaptedRunnableAction:24;java/util/concurrent/ForkJoinTask$AdaptedRunnable:java/util/concurrent/ForkJoinTask:AdaptedRunnable:24;java/util/concurrent/ForkJoinTask$AdaptedCallable:java/util/concurrent/ForkJoinTask:AdaptedCallable:24;java/util/concurrent/ForkJoinTask$AdaptedInterruptibleCallable:java/util/concurrent/ForkJoinTask:AdaptedInterruptibleCallable:24;java/util/concurrent/ForkJoinTask$RunnableExecuteAction:java/util/concurrent/ForkJoinTask:RunnableExecuteAction:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/concurrent/ForkJoinTask;java/util/concurrent/Future"]

    pub struct ForkJoinTask<V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "status", descriptor = "I", access = "package", modifiers = "volatile", is_static = false))]
        pub status: i32,
        #[cfg_attr(any(), java_field(name = "aux", descriptor = "Ljava/util/concurrent/ForkJoinTask$Aux;", access = "private", modifiers = "volatile transient", is_static = false))]
        pub aux: Object,
    }

    impl<V> ForkJoinTask<V> {
        #[cfg_attr(any(), java_field(name = "DONE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-2147483648"))]
        // static field: DONE:I
        pub fn DONE() -> i32 {
            -2147483648
        }

        #[cfg_attr(any(), java_field(name = "ABNORMAL", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "65536"))]
        // static field: ABNORMAL:I
        pub fn ABNORMAL() -> i32 {
            65536
        }

        #[cfg_attr(any(), java_field(name = "THROWN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "131072"))]
        // static field: THROWN:I
        pub fn THROWN() -> i32 {
            131072
        }

        #[cfg_attr(any(), java_field(name = "SMASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "65535"))]
        // static field: SMASK:I
        pub fn SMASK() -> i32 {
            65535
        }

        #[cfg_attr(any(), java_field(name = "UNCOMPENSATE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "65536"))]
        // static field: UNCOMPENSATE:I
        pub fn UNCOMPENSATE() -> i32 {
            65536
        }

        #[cfg_attr(any(), java_field(name = "POOLSUBMIT", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "262144"))]
        // static field: POOLSUBMIT:I
        pub fn POOLSUBMIT() -> i32 {
            262144
        }

        #[cfg_attr(any(), java_field(name = "RAN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: RAN:I
        pub fn RAN() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "INTERRUPTIBLE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: INTERRUPTIBLE:I
        pub fn INTERRUPTIBLE() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "TIMED", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: TIMED:I
        pub fn TIMED() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: U:Ljdk/internal/misc/Unsafe;
        pub fn U() -> Unsafe {
            panic!("stub: java/util/concurrent/ForkJoinTask.U:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "STATUS", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: STATUS:J
        pub fn STATUS() -> i64 {
            panic!("stub: java/util/concurrent/ForkJoinTask.STATUS:J")
        }

        #[cfg_attr(any(), java_field(name = "AUX", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: AUX:J
        pub fn AUX() -> i64 {
            panic!("stub: java/util/concurrent/ForkJoinTask.AUX:J")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-7721805057305804111"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -7721805057305804111i64
        }

        #[java_method(name = "getAndBitwiseOrStatus", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrStatus(&self, v: i32) -> Result<i32> {
            panic!("stub: java/util/concurrent/ForkJoinTask.getAndBitwiseOrStatus:(I)I")
        }

        #[java_method(name = "casStatus", descriptor = "(II)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn casStatus(&self, c: i32, v: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.casStatus:(II)Z")
        }

        #[java_method(name = "casAux", descriptor = "(Ljava/util/concurrent/ForkJoinTask$Aux;Ljava/util/concurrent/ForkJoinTask$Aux;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn casAux(&self, c: Object, v: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.casAux:(Ljava/util/concurrent/ForkJoinTask$Aux;Ljava/util/concurrent/ForkJoinTask$Aux;)Z")
        }

        #[java_method(name = "markPoolSubmission", descriptor = "()V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn markPoolSubmission(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.markPoolSubmission:()V")
        }

        #[java_method(name = "signalWaiters", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signalWaiters(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.signalWaiters:()V")
        }

        #[java_method(name = "setDone", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDone(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/ForkJoinTask.setDone:()I")
        }

        #[java_method(name = "trySetCancelled", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trySetCancelled(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/ForkJoinTask.trySetCancelled:()I")
        }

        #[java_method(name = "trySetThrown", descriptor = "(Ljava/lang/Throwable;)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trySetThrown(&self, ex: Throwable) -> Result<i32> {
            panic!("stub: java/util/concurrent/ForkJoinTask.trySetThrown:(Ljava/lang/Throwable;)I")
        }

        #[java_method(name = "trySetException", descriptor = "(Ljava/lang/Throwable;)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trySetException(&self, ex: Throwable) -> Result<i32> {
            panic!("stub: java/util/concurrent/ForkJoinTask.trySetException:(Ljava/lang/Throwable;)I")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "isExceptionalStatus", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isExceptionalStatus(s: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.isExceptionalStatus:(I)Z")
        }

        #[java_method(name = "doExec", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doExec(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/ForkJoinTask.doExec:()I")
        }

        #[java_method(name = "awaitDone", descriptor = "(IJ)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn awaitDone(&self, how: i32, deadline: i64) -> Result<i32> {
            panic!("stub: java/util/concurrent/ForkJoinTask.awaitDone:(IJ)I")
        }

        #[java_method(name = "cancelIgnoringExceptions", descriptor = "(Ljava/util/concurrent/Future;)V", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/Future<*>;)V")]
        pub fn cancelIgnoringExceptions(t: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.cancelIgnoringExceptions:(Ljava/util/concurrent/Future;)V")
        }

        #[java_method(name = "getThrowableException", descriptor = "()Ljava/lang/Throwable;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getThrowableException(&self) -> Result<Throwable> {
            panic!("stub: java/util/concurrent/ForkJoinTask.getThrowableException:()Ljava/lang/Throwable;")
        }

        #[java_method(name = "getException", descriptor = "(I)Ljava/lang/Throwable;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getException_i(&self, s: i32) -> Result<Throwable> {
            panic!("stub: java/util/concurrent/ForkJoinTask.getException:(I)Ljava/lang/Throwable;")
        }

        #[java_method(name = "reportException", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reportException(&self, s: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.reportException:(I)V")
        }

        #[java_method(name = "reportExecutionException", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reportExecutionException(&self, s: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.reportExecutionException:(I)V")
        }

        #[java_method(name = "rethrow", descriptor = "(Ljava/lang/Throwable;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rethrow(ex: Throwable) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.rethrow:(Ljava/lang/Throwable;)V")
        }

        #[java_method(name = "uncheckedThrow", descriptor = "(Ljava/lang/Throwable;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/Throwable", generic_signature = "<T:Ljava/lang/Throwable;>(Ljava/lang/Throwable;)V^TT;")]
        pub fn uncheckedThrow(t: Throwable) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.uncheckedThrow:(Ljava/lang/Throwable;)V")
        }

        #[java_method(name = "fork", descriptor = "()Ljava/util/concurrent/ForkJoinTask;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/ForkJoinTask<TV;>;")]
        pub fn fork(&self) -> Result<ForkJoinTask<Object>> {
            panic!("stub: java/util/concurrent/ForkJoinTask.fork:()Ljava/util/concurrent/ForkJoinTask;")
        }

        #[java_method(name = "join", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TV;")]
        pub fn join(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ForkJoinTask.join:()Ljava/lang/Object;")
        }

        #[java_method(name = "invoke", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TV;")]
        pub fn invoke(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ForkJoinTask.invoke:()Ljava/lang/Object;")
        }

        #[java_method(name = "invokeAll", descriptor = "(Ljava/util/concurrent/ForkJoinTask;Ljava/util/concurrent/ForkJoinTask;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/ForkJoinTask<*>;Ljava/util/concurrent/ForkJoinTask<*>;)V")]
        pub fn invokeAll_forkjo_forkjo(t1: ForkJoinTask<Object>, t2: ForkJoinTask<Object>) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.invokeAll:(Ljava/util/concurrent/ForkJoinTask;Ljava/util/concurrent/ForkJoinTask;)V")
        }

        #[java_method(name = "invokeAll", descriptor = "([Ljava/util/concurrent/ForkJoinTask;)V", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/util/concurrent/ForkJoinTask<*>;)V")]
        pub fn invokeAll_arr_for(tasks: Rc<RefCell<Vec<ForkJoinTask<Object>>>>) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.invokeAll:([Ljava/util/concurrent/ForkJoinTask;)V")
        }

        #[java_method(name = "invokeAll", descriptor = "(Ljava/util/Collection;)Ljava/util/Collection;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/util/concurrent/ForkJoinTask<*>;>(Ljava/util/Collection<TT;>;)Ljava/util/Collection<TT;>;")]
        pub fn invokeAll_coll(tasks: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ForkJoinTask.invokeAll:(Ljava/util/Collection;)Ljava/util/Collection;")
        }

        #[java_method(name = "cancel", descriptor = "(Z)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn cancel(&self, mayInterruptIfRunning: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.cancel:(Z)Z")
        }

        #[java_method(name = "isDone", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDone(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.isDone:()Z")
        }

        #[java_method(name = "isCancelled", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCancelled(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.isCancelled:()Z")
        }

        #[java_method(name = "isCompletedAbnormally", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCompletedAbnormally(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.isCompletedAbnormally:()Z")
        }

        #[java_method(name = "isCompletedNormally", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCompletedNormally(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.isCompletedNormally:()Z")
        }

        #[java_method(name = "state", descriptor = "()Ljava/util/concurrent/Future$State;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn state(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ForkJoinTask.state:()Ljava/util/concurrent/Future$State;")
        }

        #[java_method(name = "resultNow", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TV;")]
        pub fn resultNow(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ForkJoinTask.resultNow:()Ljava/lang/Object;")
        }

        #[java_method(name = "exceptionNow", descriptor = "()Ljava/lang/Throwable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exceptionNow(&self) -> Result<Throwable> {
            panic!("stub: java/util/concurrent/ForkJoinTask.exceptionNow:()Ljava/lang/Throwable;")
        }

        #[java_method(name = "getException", descriptor = "()Ljava/lang/Throwable;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getException(&self) -> Result<Throwable> {
            panic!("stub: java/util/concurrent/ForkJoinTask.getException:()Ljava/lang/Throwable;")
        }

        #[java_method(name = "completeExceptionally", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn completeExceptionally(&self, ex: Throwable) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.completeExceptionally:(Ljava/lang/Throwable;)V")
        }

        #[java_method(name = "complete", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TV;)V")]
        pub fn complete(&self, value: V) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.complete:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "quietlyComplete", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn quietlyComplete(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.quietlyComplete:()V")
        }

        #[java_method(name = "get", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException,java/util/concurrent/ExecutionException", generic_signature = "()TV;")]
        pub fn get(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ForkJoinTask.get:()Ljava/lang/Object;")
        }

        #[java_method(name = "get", descriptor = "(JLjava/util/concurrent/TimeUnit;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException,java/util/concurrent/ExecutionException,java/util/concurrent/TimeoutException", generic_signature = "(JLjava/util/concurrent/TimeUnit;)TV;")]
        pub fn get_l_timeun(&self, timeout: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/concurrent/ForkJoinTask.get:(JLjava/util/concurrent/TimeUnit;)Ljava/lang/Object;")
        }

        #[java_method(name = "quietlyJoin", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn quietlyJoin(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.quietlyJoin:()V")
        }

        #[java_method(name = "quietlyInvoke", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn quietlyInvoke(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.quietlyInvoke:()V")
        }

        #[java_method(name = "quietlyJoin", descriptor = "(JLjava/util/concurrent/TimeUnit;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InterruptedException")]
        pub fn quietlyJoin_l_timeun(&self, timeout: i64, arg1: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.quietlyJoin:(JLjava/util/concurrent/TimeUnit;)Z")
        }

        #[java_method(name = "quietlyJoinUninterruptibly", descriptor = "(JLjava/util/concurrent/TimeUnit;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn quietlyJoinUninterruptibly(&self, timeout: i64, arg1: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.quietlyJoinUninterruptibly:(JLjava/util/concurrent/TimeUnit;)Z")
        }

        #[java_method(name = "helpQuiesce", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn helpQuiesce() -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.helpQuiesce:()V")
        }

        #[java_method(name = "reinitialize", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reinitialize(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.reinitialize:()V")
        }

        #[java_method(name = "getPool", descriptor = "()Ljava/util/concurrent/ForkJoinPool;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPool() -> Result<Object> {
            panic!("stub: java/util/concurrent/ForkJoinTask.getPool:()Ljava/util/concurrent/ForkJoinPool;")
        }

        #[java_method(name = "inForkJoinPool", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inForkJoinPool() -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.inForkJoinPool:()Z")
        }

        #[java_method(name = "tryUnfork", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryUnfork(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.tryUnfork:()Z")
        }

        #[java_method(name = "getQueuedTaskCount", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getQueuedTaskCount() -> Result<i32> {
            panic!("stub: java/util/concurrent/ForkJoinTask.getQueuedTaskCount:()I")
        }

        #[java_method(name = "getSurplusQueuedTaskCount", descriptor = "()I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSurplusQueuedTaskCount() -> Result<i32> {
            panic!("stub: java/util/concurrent/ForkJoinTask.getSurplusQueuedTaskCount:()I")
        }

        #[java_method(name = "getRawResult", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()TV;")]
        pub fn getRawResult(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ForkJoinTask.getRawResult:()Ljava/lang/Object;")
        }

        #[java_method(name = "setRawResult", descriptor = "(Ljava/lang/Object;)V", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TV;)V")]
        pub fn setRawResult(&self, arg0: V) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.setRawResult:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "exec", descriptor = "()Z", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn exec(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.exec:()Z")
        }

        #[java_method(name = "peekNextLocalTask", descriptor = "()Ljava/util/concurrent/ForkJoinTask;", access = "protected", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/ForkJoinTask<*>;")]
        pub fn peekNextLocalTask() -> Result<ForkJoinTask<Object>> {
            panic!("stub: java/util/concurrent/ForkJoinTask.peekNextLocalTask:()Ljava/util/concurrent/ForkJoinTask;")
        }

        #[java_method(name = "pollNextLocalTask", descriptor = "()Ljava/util/concurrent/ForkJoinTask;", access = "protected", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/ForkJoinTask<*>;")]
        pub fn pollNextLocalTask() -> Result<ForkJoinTask<Object>> {
            panic!("stub: java/util/concurrent/ForkJoinTask.pollNextLocalTask:()Ljava/util/concurrent/ForkJoinTask;")
        }

        #[java_method(name = "pollTask", descriptor = "()Ljava/util/concurrent/ForkJoinTask;", access = "protected", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/ForkJoinTask<*>;")]
        pub fn pollTask() -> Result<ForkJoinTask<Object>> {
            panic!("stub: java/util/concurrent/ForkJoinTask.pollTask:()Ljava/util/concurrent/ForkJoinTask;")
        }

        #[java_method(name = "pollSubmission", descriptor = "()Ljava/util/concurrent/ForkJoinTask;", access = "protected", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/ForkJoinTask<*>;")]
        pub fn pollSubmission() -> Result<ForkJoinTask<Object>> {
            panic!("stub: java/util/concurrent/ForkJoinTask.pollSubmission:()Ljava/util/concurrent/ForkJoinTask;")
        }

        #[java_method(name = "getForkJoinTaskTag", descriptor = "()S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getForkJoinTaskTag(&self) -> Result<i16> {
            panic!("stub: java/util/concurrent/ForkJoinTask.getForkJoinTaskTag:()S")
        }

        #[java_method(name = "setForkJoinTaskTag", descriptor = "(S)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setForkJoinTaskTag(&self, newValue: i16) -> Result<i16> {
            panic!("stub: java/util/concurrent/ForkJoinTask.setForkJoinTaskTag:(S)S")
        }

        #[java_method(name = "compareAndSetForkJoinTaskTag", descriptor = "(SS)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetForkJoinTaskTag(&self, expect: i16, update: i16) -> Result<bool> {
            panic!("stub: java/util/concurrent/ForkJoinTask.compareAndSetForkJoinTaskTag:(SS)Z")
        }

        #[java_method(name = "adapt", descriptor = "(Ljava/lang/Runnable;)Ljava/util/concurrent/ForkJoinTask;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Runnable;)Ljava/util/concurrent/ForkJoinTask<*>;")]
        pub fn adapt_runnab(runnable: Object) -> Result<ForkJoinTask<Object>> {
            panic!("stub: java/util/concurrent/ForkJoinTask.adapt:(Ljava/lang/Runnable;)Ljava/util/concurrent/ForkJoinTask;")
        }

        #[java_method(name = "adapt", descriptor = "(Ljava/lang/Runnable;Ljava/lang/Object;)Ljava/util/concurrent/ForkJoinTask;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/lang/Runnable;TT;)Ljava/util/concurrent/ForkJoinTask<TT;>;")]
        pub fn adapt_runnab_obj(runnable: Object, result: Object) -> Result<ForkJoinTask<Object>> {
            panic!("stub: java/util/concurrent/ForkJoinTask.adapt:(Ljava/lang/Runnable;Ljava/lang/Object;)Ljava/util/concurrent/ForkJoinTask;")
        }

        #[java_method(name = "adapt", descriptor = "(Ljava/util/concurrent/Callable;)Ljava/util/concurrent/ForkJoinTask;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/concurrent/Callable<+TT;>;)Ljava/util/concurrent/ForkJoinTask<TT;>;")]
        pub fn adapt_callab(callable: Object) -> Result<ForkJoinTask<Object>> {
            panic!("stub: java/util/concurrent/ForkJoinTask.adapt:(Ljava/util/concurrent/Callable;)Ljava/util/concurrent/ForkJoinTask;")
        }

        #[java_method(name = "adaptInterruptible", descriptor = "(Ljava/util/concurrent/Callable;)Ljava/util/concurrent/ForkJoinTask;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/concurrent/Callable<+TT;>;)Ljava/util/concurrent/ForkJoinTask<TT;>;")]
        pub fn adaptInterruptible(callable: Object) -> Result<ForkJoinTask<Object>> {
            panic!("stub: java/util/concurrent/ForkJoinTask.adaptInterruptible:(Ljava/util/concurrent/Callable;)Ljava/util/concurrent/ForkJoinTask;")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ForkJoinTask.readObject:(Ljava/io/ObjectInputStream;)V")
        }
    }
}
