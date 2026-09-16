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

impl<T: Clone + Default + 'static> From<CountedCompleter<T>> for ForkJoinTask<T> {
    fn from(v: CountedCompleter<T>) -> ForkJoinTask<T> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/CountedCompleter"]
    #[super_class       = "java/util/concurrent/ForkJoinTask"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/util/concurrent/ForkJoinTask<TT;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CountedCompleter.java"]
    #[inner_classes     = "java/util/concurrent/ForkJoinPool$WorkQueue:java/util/concurrent/ForkJoinPool:WorkQueue:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ForkJoinTask<T>"]
    #[superclass_fields(status: i32, aux: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/concurrent/CountedCompleter;java/util/concurrent/ForkJoinTask;java/util/concurrent/Future"]

    pub struct CountedCompleter<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "completer", descriptor = "Ljava/util/concurrent/CountedCompleter;", access = "package", modifiers = "final", is_static = false, generic_signature = "Ljava/util/concurrent/CountedCompleter<*>;"))]
        pub completer: CountedCompleter<Object>,
        #[cfg_attr(any(), java_field(name = "pending", descriptor = "I", access = "package", modifiers = "volatile", is_static = false))]
        pub pending: i32,
    }

    impl<T> CountedCompleter<T> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "5232453752276485070"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            5232453752276485070i64
        }

        #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: U:Ljdk/internal/misc/Unsafe;
        pub fn U() -> Unsafe {
            panic!("stub: java/util/concurrent/CountedCompleter.U:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "PENDING", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: PENDING:J
        pub fn PENDING() -> i64 {
            panic!("stub: java/util/concurrent/CountedCompleter.PENDING:J")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/concurrent/CountedCompleter;I)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/CountedCompleter<*>;I)V")]
        pub fn new_counte_i(completer: CountedCompleter<Object>, initialPendingCount: i32) -> Result<Self> {
            panic!("stub: java/util/concurrent/CountedCompleter.<init>:(Ljava/util/concurrent/CountedCompleter;I)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/concurrent/CountedCompleter;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/CountedCompleter<*>;)V")]
        // java: <init>(Ljava/util/concurrent/CountedCompleter;)V
        pub fn new_counte(mut completer: CountedCompleter<Object>) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ForkJoinTask::new()?);
            this.__set_completer(Clone::clone(&completer));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/concurrent/CountedCompleter.<init>:()V")
        }

        #[java_method(name = "compute", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn compute(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.compute:()V")
        }

        #[java_method(name = "onCompletion", descriptor = "(Ljava/util/concurrent/CountedCompleter;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/CountedCompleter<*>;)V")]
        pub fn onCompletion(&self, caller: CountedCompleter<Object>) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.onCompletion:(Ljava/util/concurrent/CountedCompleter;)V")
        }

        #[java_method(name = "onExceptionalCompletion", descriptor = "(Ljava/lang/Throwable;Ljava/util/concurrent/CountedCompleter;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Throwable;Ljava/util/concurrent/CountedCompleter<*>;)Z")]
        pub fn onExceptionalCompletion(&self, ex: Throwable, caller: CountedCompleter<Object>) -> Result<bool> {
            panic!("stub: java/util/concurrent/CountedCompleter.onExceptionalCompletion:(Ljava/lang/Throwable;Ljava/util/concurrent/CountedCompleter;)Z")
        }

        #[java_method(name = "getCompleter", descriptor = "()Ljava/util/concurrent/CountedCompleter;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/CountedCompleter<*>;")]
        pub fn getCompleter(&self) -> Result<CountedCompleter<Object>> {
            panic!("stub: java/util/concurrent/CountedCompleter.getCompleter:()Ljava/util/concurrent/CountedCompleter;")
        }

        #[java_method(name = "getPendingCount", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPendingCount(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/CountedCompleter.getPendingCount:()I")
        }

        #[java_method(name = "setPendingCount", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPendingCount(&self, count: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.setPendingCount:(I)V")
        }

        #[java_method(name = "addToPendingCount", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addToPendingCount(&self, delta: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.addToPendingCount:(I)V")
        }

        #[java_method(name = "compareAndSetPendingCount", descriptor = "(II)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetPendingCount(&self, expected: i32, count: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/CountedCompleter.compareAndSetPendingCount:(II)Z")
        }

        #[java_method(name = "weakCompareAndSetPendingCount", descriptor = "(II)Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetPendingCount(&self, expected: i32, count: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/CountedCompleter.weakCompareAndSetPendingCount:(II)Z")
        }

        #[java_method(name = "decrementPendingCountUnlessZero", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decrementPendingCountUnlessZero(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/CountedCompleter.decrementPendingCountUnlessZero:()I")
        }

        #[java_method(name = "getRoot", descriptor = "()Ljava/util/concurrent/CountedCompleter;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/CountedCompleter<*>;")]
        pub fn getRoot(&self) -> Result<CountedCompleter<Object>> {
            panic!("stub: java/util/concurrent/CountedCompleter.getRoot:()Ljava/util/concurrent/CountedCompleter;")
        }

        #[java_method(name = "tryComplete", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryComplete(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.tryComplete:()V")
        }

        #[java_method(name = "propagateCompletion", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn propagateCompletion(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.propagateCompletion:()V")
        }

        #[java_method(name = "complete", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)V")]
        pub fn complete(&self, rawResult: T) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.complete:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "firstComplete", descriptor = "()Ljava/util/concurrent/CountedCompleter;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/CountedCompleter<*>;")]
        pub fn firstComplete(&self) -> Result<CountedCompleter<Object>> {
            panic!("stub: java/util/concurrent/CountedCompleter.firstComplete:()Ljava/util/concurrent/CountedCompleter;")
        }

        #[java_method(name = "nextComplete", descriptor = "()Ljava/util/concurrent/CountedCompleter;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/CountedCompleter<*>;")]
        pub fn nextComplete(&self) -> Result<CountedCompleter<Object>> {
            panic!("stub: java/util/concurrent/CountedCompleter.nextComplete:()Ljava/util/concurrent/CountedCompleter;")
        }

        #[java_method(name = "quietlyCompleteRoot", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn quietlyCompleteRoot(&self) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.quietlyCompleteRoot:()V")
        }

        #[java_method(name = "helpComplete", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn helpComplete(&self, maxTasks: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.helpComplete:(I)V")
        }

        #[java_method(name = "trySetException", descriptor = "(Ljava/lang/Throwable;)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trySetException(&self, ex: Throwable) -> Result<i32> {
            panic!("stub: java/util/concurrent/CountedCompleter.trySetException:(Ljava/lang/Throwable;)I")
        }

        #[java_method(name = "exec", descriptor = "()Z", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exec(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/CountedCompleter.exec:()Z")
        }

        #[java_method(name = "getRawResult", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT;")]
        pub fn getRawResult(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/CountedCompleter.getRawResult:()Ljava/lang/Object;")
        }

        #[java_method(name = "setRawResult", descriptor = "(Ljava/lang/Object;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)V")]
        pub fn setRawResult(&self, t: T) -> Result<()> {
            panic!("stub: java/util/concurrent/CountedCompleter.setRawResult:(Ljava/lang/Object;)V")
        }
    }
}
