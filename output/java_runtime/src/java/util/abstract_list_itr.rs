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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/AbstractList$Itr"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Iterator"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Object;Ljava/util/Iterator<TE;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractList.java"]
    #[inner_classes     = "java/util/AbstractList$Itr:java/util/AbstractList:Itr:2"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/AbstractList$Itr;java/util/Iterator"]

    pub struct AbstractList_Itr {
        #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I", is_static = false))]
        pub cursor: i32,
        #[cfg_attr(any(), java_field(name = "lastRet", descriptor = "I", is_static = false))]
        pub lastRet: i32,
        #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I", is_static = false))]
        pub expectedModCount: i32,
        #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/AbstractList;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub this_0: AbstractList<Object>,
    }

    impl AbstractList_Itr {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":4112")]
        pub fn new(mut arg_0: AbstractList<Object>) -> Result<Self> {
            let mut this = Self::default();
            this.__set_this_0(Clone::clone(&arg_0));
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_cursor(0i32);
            this.__set_lastRet(-1i32);
            this.__set_expectedModCount(this.__get_this_0().__get_modCount());
            Ok(this)
        }

        #[java_method(name = "hasNext", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasNext(&self) -> Result<bool> {
            panic!("stub: java/util/AbstractList$Itr.hasNext:()Z")
        }

        #[java_method(name = "next", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn next(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractList$Itr.next:()Ljava/lang/Object;")
        }

        #[java_method(name = "remove", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove(&self) -> Result<()> {
            panic!("stub: java/util/AbstractList$Itr.remove:()V")
        }

        #[java_method(name = "checkForComodification", descriptor = "()V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkForComodification(&self) -> Result<()> {
            panic!("stub: java/util/AbstractList$Itr.checkForComodification:()V")
        }

        #[java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE;>;)V")]
        pub fn forEachRemaining(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/AbstractList$Itr.forEachRemaining:(Ljava/util/function/Consumer;)V")
        }
    }
}
