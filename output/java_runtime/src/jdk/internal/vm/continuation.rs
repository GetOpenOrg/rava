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
use crate::jdk::internal::vm::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::misc::Unsafe;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/vm/Continuation"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Continuation.java"]
    #[inner_classes     = "jdk/internal/vm/Continuation$Pinned:jdk/internal/vm/Continuation:Pinned:16409;java/lang/StackWalker$Option:java/lang/StackWalker:Option:16409;jdk/internal/vm/Continuation$PreemptStatus:jdk/internal/vm/Continuation:PreemptStatus:16409;java/lang/StackWalker$StackFrame:java/lang/StackWalker:StackFrame:1545;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/vm/Continuation"]
    #[has_to_string_method = true]

    pub struct Continuation {
        #[cfg_attr(any(), java_field(name = "target", descriptor = "Ljava/lang/Runnable;", access = "private", modifiers = "final", is_static = false))]
        pub target: Object,
        #[cfg_attr(any(), java_field(name = "scope", descriptor = "Ljdk/internal/vm/ContinuationScope;", access = "private", modifiers = "final", is_static = false))]
        pub scope: Object,
        #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljdk/internal/vm/Continuation;", access = "private", modifiers = "", is_static = false))]
        pub parent: Continuation,
        #[cfg_attr(any(), java_field(name = "child", descriptor = "Ljdk/internal/vm/Continuation;", access = "private", modifiers = "", is_static = false))]
        pub child: Continuation,
        #[cfg_attr(any(), java_field(name = "tail", descriptor = "Ljdk/internal/vm/StackChunk;", access = "private", modifiers = "", is_static = false))]
        pub tail: Object,
        #[cfg_attr(any(), java_field(name = "done", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub done: bool,
        #[cfg_attr(any(), java_field(name = "mounted", descriptor = "Z", access = "private", modifiers = "volatile", is_static = false))]
        pub mounted: bool,
        #[cfg_attr(any(), java_field(name = "yieldInfo", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "", is_static = false))]
        pub yieldInfo: Object,
        #[cfg_attr(any(), java_field(name = "preempted", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub preempted: bool,
        #[cfg_attr(any(), java_field(name = "scopedValueCache", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "", is_static = false))]
        pub scopedValueCache: Rc<RefCell<Vec<Object>>>,
    }

    impl Continuation {
        #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: U:Ljdk/internal/misc/Unsafe;
        pub fn U() -> Unsafe {
            panic!("stub: jdk/internal/vm/Continuation.U:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "MOUNTED_OFFSET", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: MOUNTED_OFFSET:J
        pub fn MOUNTED_OFFSET() -> i64 {
            panic!("stub: jdk/internal/vm/Continuation.MOUNTED_OFFSET:J")
        }

        #[cfg_attr(any(), java_field(name = "PRESERVE_SCOPED_VALUE_CACHE", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
        // static field: PRESERVE_SCOPED_VALUE_CACHE:Z
        pub fn PRESERVE_SCOPED_VALUE_CACHE() -> bool {
            false
        }

        #[cfg_attr(any(), java_field(name = "JLA", descriptor = "Ljdk/internal/access/JavaLangAccess;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JLA:Ljdk/internal/access/JavaLangAccess;
        pub fn JLA() -> Object {
            panic!("stub: jdk/internal/vm/Continuation.JLA:Ljdk/internal/access/JavaLangAccess;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "pinnedReason", descriptor = "(I)Ljdk/internal/vm/Continuation$Pinned;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pinnedReason(reason: i32) -> Result<Object> {
            panic!("stub: jdk/internal/vm/Continuation.pinnedReason:(I)Ljdk/internal/vm/Continuation$Pinned;")
        }

        #[java_method(name = "currentCarrierThread", descriptor = "()Ljava/lang/Thread;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn currentCarrierThread() -> Result<Thread> {
            panic!("stub: jdk/internal/vm/Continuation.currentCarrierThread:()Ljava/lang/Thread;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljdk/internal/vm/ContinuationScope;Ljava/lang/Runnable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(scope: Object, target: Object) -> Result<Self> {
            panic!("stub: jdk/internal/vm/Continuation.<init>:(Ljdk/internal/vm/ContinuationScope;Ljava/lang/Runnable;)V")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "getScope", descriptor = "()Ljdk/internal/vm/ContinuationScope;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getScope(&self) -> Result<Object> {
            panic!("stub: jdk/internal/vm/Continuation.getScope:()Ljdk/internal/vm/ContinuationScope;")
        }

        #[java_method(name = "getParent", descriptor = "()Ljdk/internal/vm/Continuation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getParent(&self) -> Result<Continuation> {
            panic!("stub: jdk/internal/vm/Continuation.getParent:()Ljdk/internal/vm/Continuation;")
        }

        #[java_method(name = "getCurrentContinuation", descriptor = "(Ljdk/internal/vm/ContinuationScope;)Ljdk/internal/vm/Continuation;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrentContinuation(scope: Object) -> Result<Continuation> {
            panic!("stub: jdk/internal/vm/Continuation.getCurrentContinuation:(Ljdk/internal/vm/ContinuationScope;)Ljdk/internal/vm/Continuation;")
        }

        #[java_method(name = "stackWalker", descriptor = "()Ljava/lang/StackWalker;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stackWalker(&self) -> Result<Object> {
            panic!("stub: jdk/internal/vm/Continuation.stackWalker:()Ljava/lang/StackWalker;")
        }

        #[java_method(name = "stackWalker", descriptor = "(Ljava/util/Set;)Ljava/lang/StackWalker;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Set<Ljava/lang/StackWalker$Option;>;)Ljava/lang/StackWalker;")]
        pub fn stackWalker_set(&self, options: Object) -> Result<Object> {
            panic!("stub: jdk/internal/vm/Continuation.stackWalker:(Ljava/util/Set;)Ljava/lang/StackWalker;")
        }

        #[java_method(name = "stackWalker", descriptor = "(Ljava/util/Set;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Set<Ljava/lang/StackWalker$Option;>;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;")]
        pub fn stackWalker_set_contin(&self, options: Object, scope: Object) -> Result<Object> {
            panic!("stub: jdk/internal/vm/Continuation.stackWalker:(Ljava/util/Set;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;")
        }

        #[java_method(name = "getStackTrace", descriptor = "()[Ljava/lang/StackTraceElement;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getStackTrace(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: jdk/internal/vm/Continuation.getStackTrace:()[Ljava/lang/StackTraceElement;")
        }

        #[java_method(name = "wrapWalk", descriptor = "(Ljdk/internal/vm/Continuation;Ljdk/internal/vm/ContinuationScope;Ljava/util/function/Supplier;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljdk/internal/vm/Continuation;Ljdk/internal/vm/ContinuationScope;Ljava/util/function/Supplier<TR;>;)TR;")]
        pub fn wrapWalk(inner: Continuation, scope: Object, walk: Object) -> Result<Object> {
            panic!("stub: jdk/internal/vm/Continuation.wrapWalk:(Ljdk/internal/vm/Continuation;Ljdk/internal/vm/ContinuationScope;Ljava/util/function/Supplier;)Ljava/lang/Object;")
        }

        #[java_method(name = "innermost", descriptor = "()Ljdk/internal/vm/Continuation;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn innermost(&self) -> Result<Continuation> {
            panic!("stub: jdk/internal/vm/Continuation.innermost:()Ljdk/internal/vm/Continuation;")
        }

        #[java_method(name = "mount", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mount(&self) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.mount:()V")
        }

        #[java_method(name = "unmount", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unmount(&self) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.unmount:()V")
        }

        #[java_method(name = "run", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn run(&self) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.run:()V")
        }

        #[java_method(name = "postYieldCleanup", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn postYieldCleanup(&self) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.postYieldCleanup:()V")
        }

        #[java_method(name = "finish", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn finish(&self) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.finish:()V")
        }

        #[native]
        #[java_native(name = "doYield", descriptor = "()I", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn doYield() -> Result<i32> {
            panic!("native: jdk/internal/vm/Continuation.doYield:()I")
        }

        #[native]
        #[java_native(name = "enterSpecial", descriptor = "(Ljdk/internal/vm/Continuation;ZZ)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn enterSpecial(arg0: Continuation, arg1: bool, arg2: bool) -> Result<()> {
            panic!("native: jdk/internal/vm/Continuation.enterSpecial:(Ljdk/internal/vm/Continuation;ZZ)V")
        }

        #[java_method(name = "enter", descriptor = "(Ljdk/internal/vm/Continuation;Z)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn enter(c: Continuation, isContinue: bool) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.enter:(Ljdk/internal/vm/Continuation;Z)V")
        }

        #[java_method(name = "enter0", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn enter0(&self) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.enter0:()V")
        }

        #[java_method(name = "isStarted", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isStarted(&self) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.isStarted:()Z")
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.isEmpty:()Z")
        }

        #[java_method(name = "yield", descriptor = "(Ljdk/internal/vm/ContinuationScope;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn yield_(scope: Object) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.yield:(Ljdk/internal/vm/ContinuationScope;)Z")
        }

        #[java_method(name = "yield0", descriptor = "(Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn yield0(&self, scope: Object, child: Continuation) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.yield0:(Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)Z")
        }

        #[java_method(name = "onPinned0", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn onPinned0(&self, reason: i32) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.onPinned0:(I)V")
        }

        #[java_method(name = "onPinned", descriptor = "(Ljdk/internal/vm/Continuation$Pinned;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn onPinned(&self, reason: Object) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.onPinned:(Ljdk/internal/vm/Continuation$Pinned;)V")
        }

        #[java_method(name = "onContinue", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn onContinue(&self) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.onContinue:()V")
        }

        #[java_method(name = "isDone", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDone(&self) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.isDone:()Z")
        }

        #[java_method(name = "isPreempted", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPreempted(&self) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.isPreempted:()Z")
        }

        #[native]
        #[java_native(name = "pin", descriptor = "()V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn pin() -> Result<()> {
            panic!("native: jdk/internal/vm/Continuation.pin:()V")
        }

        #[native]
        #[java_native(name = "unpin", descriptor = "()V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn unpin() -> Result<()> {
            panic!("native: jdk/internal/vm/Continuation.unpin:()V")
        }

        #[java_method(name = "isPinned", descriptor = "(Ljdk/internal/vm/ContinuationScope;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPinned(scope: Object) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.isPinned:(Ljdk/internal/vm/ContinuationScope;)Z")
        }

        #[native]
        #[java_native(name = "isPinned0", descriptor = "(Ljdk/internal/vm/ContinuationScope;)I", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn isPinned0(arg0: Object) -> Result<i32> {
            panic!("native: jdk/internal/vm/Continuation.isPinned0:(Ljdk/internal/vm/ContinuationScope;)I")
        }

        #[java_method(name = "fence", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fence(&self) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.fence:()Z")
        }

        #[java_method(name = "compareAndSetMounted", descriptor = "(ZZ)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetMounted(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.compareAndSetMounted:(ZZ)Z")
        }

        #[java_method(name = "setMounted", descriptor = "(Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMounted(&self, newValue: bool) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.setMounted:(Z)V")
        }

        #[java_method(name = "id", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn id(&self) -> Result<String> {
            panic!("stub: jdk/internal/vm/Continuation.id:()Ljava/lang/String;")
        }

        #[java_method(name = "tryPreempt", descriptor = "(Ljava/lang/Thread;)Ljdk/internal/vm/Continuation$PreemptStatus;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryPreempt(&self, thread: Thread) -> Result<Object> {
            panic!("stub: jdk/internal/vm/Continuation.tryPreempt:(Ljava/lang/Thread;)Ljdk/internal/vm/Continuation$PreemptStatus;")
        }

        #[native]
        #[java_native(name = "registerNatives", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn registerNatives() -> Result<()> {
            panic!("native: jdk/internal/vm/Continuation.registerNatives:()V")
        }

        #[java_method(name = "dump", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dump(&self) -> Result<()> {
            panic!("stub: jdk/internal/vm/Continuation.dump:()V")
        }

        #[java_method(name = "isEmptyOrTrue", descriptor = "(Ljava/lang/String;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmptyOrTrue(property: String) -> Result<bool> {
            panic!("stub: jdk/internal/vm/Continuation.isEmptyOrTrue:(Ljava/lang/String;)Z")
        }
    }
}
