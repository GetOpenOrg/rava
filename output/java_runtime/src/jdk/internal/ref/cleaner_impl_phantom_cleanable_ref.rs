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
use crate::jdk::internal::r#ref::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::r#ref::PhantomCleanable;

impl From<CleanerImpl_PhantomCleanableRef> for PhantomCleanable<Object> {
    fn from(v: CleanerImpl_PhantomCleanableRef) -> PhantomCleanable<Object> { v.__into_super() }
}

impl From<CleanerImpl_PhantomCleanableRef> for PhantomReference<Object> {
    fn from(v: CleanerImpl_PhantomCleanableRef) -> PhantomReference<Object> { v.__into_super().__into_super() }
}

impl From<CleanerImpl_PhantomCleanableRef> for Reference<Object> {
    fn from(v: CleanerImpl_PhantomCleanableRef) -> Reference<Object> { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/ref/CleanerImpl$PhantomCleanableRef"]
    #[super_class       = "jdk/internal/ref/PhantomCleanable"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljdk/internal/ref/PhantomCleanable<Ljava/lang/Object;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CleanerImpl.java"]
    #[inner_classes     = "jdk/internal/ref/CleanerImpl$PhantomCleanableRef:jdk/internal/ref/CleanerImpl:PhantomCleanableRef:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "PhantomCleanable<Object>"]
    #[superclass_fields(referent: Object, queue: ReferenceQueue<Object>, next: Reference<Object>, discovered: Reference<Object>, prev: PhantomCleanable<Object>, list: PhantomCleanable<Object>)]
    #[all_supertypes    = "java/lang/Object;java/lang/ref/Cleaner$Cleanable;java/lang/ref/PhantomReference;java/lang/ref/Reference;jdk/internal/ref/CleanerImpl$PhantomCleanableRef;jdk/internal/ref/PhantomCleanable"]

    pub struct CleanerImpl_PhantomCleanableRef {
        #[cfg_attr(any(), java_field(name = "action", descriptor = "Ljava/lang/Runnable;", access = "private", modifiers = "final", is_static = false))]
        pub action: Object,
    }

    impl CleanerImpl_PhantomCleanableRef {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/ref/Cleaner;Ljava/lang/Runnable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_obj_cleane_runnab(obj: Object, cleaner: Cleaner, action: Object) -> Result<Self> {
            panic!("stub: jdk/internal/ref/CleanerImpl$PhantomCleanableRef.<init>:(Ljava/lang/Object;Ljava/lang/ref/Cleaner;Ljava/lang/Runnable;)V")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/ref/CleanerImpl$PhantomCleanableRef.<init>:()V")
        }

        #[java_method(name = "performCleanup", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn performCleanup(&self) -> Result<()> {
            panic!("stub: jdk/internal/ref/CleanerImpl$PhantomCleanableRef.performCleanup:()V")
        }

        #[java_method(name = "get", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self) -> Result<Object> {
            panic!("stub: jdk/internal/ref/CleanerImpl$PhantomCleanableRef.get:()Ljava/lang/Object;")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: jdk/internal/ref/CleanerImpl$PhantomCleanableRef.clear:()V")
        }
    }
}
