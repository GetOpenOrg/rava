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

impl From<AbstractList_ListItr> for AbstractList_Itr {
    fn from(v: AbstractList_ListItr) -> AbstractList_Itr { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/AbstractList$ListItr"]
    #[super_class       = "java/util/AbstractList$Itr"]
    #[interfaces        = "java/util/ListIterator"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/util/AbstractList<TE;>.Itr;Ljava/util/ListIterator<TE;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractList.java"]
    #[inner_classes     = "java/util/AbstractList$ListItr:java/util/AbstractList:ListItr:2;java/util/AbstractList$Itr:java/util/AbstractList:Itr:2"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractList_Itr"]
    #[superclass_fields(cursor: i32, lastRet: i32, expectedModCount: i32, this_0: AbstractList<Object>)]
    #[all_supertypes    = "java/lang/Object;java/util/AbstractList$Itr;java/util/AbstractList$ListItr;java/util/Iterator;java/util/ListIterator"]

    pub struct AbstractList_ListItr {
        #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/AbstractList;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub this_0: AbstractList<Object>,
    }

    impl AbstractList_ListItr {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":4112;:0")]
        pub fn new(mut arg_0: AbstractList<Object>, mut index: i32) -> Result<Self> {
            let mut this = Self::default();
            this.__set_this_0(Clone::clone(&arg_0));
            this = Self::__new_with_super(AbstractList_Itr::new(Clone::clone(&arg_0))?);
            this.__set_cursor(index);
            Ok(this)
        }

        #[java_method(name = "hasPrevious", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasPrevious(&self) -> Result<bool> {
            panic!("stub: java/util/AbstractList$ListItr.hasPrevious:()Z")
        }

        #[java_method(name = "previous", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn previous(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractList$ListItr.previous:()Ljava/lang/Object;")
        }

        #[java_method(name = "nextIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextIndex(&self) -> Result<i32> {
            panic!("stub: java/util/AbstractList$ListItr.nextIndex:()I")
        }

        #[java_method(name = "previousIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn previousIndex(&self) -> Result<i32> {
            panic!("stub: java/util/AbstractList$ListItr.previousIndex:()I")
        }

        #[java_method(name = "set", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn set(&self, e: Object) -> Result<()> {
            panic!("stub: java/util/AbstractList$ListItr.set:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "add", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn add(&self, e: Object) -> Result<()> {
            panic!("stub: java/util/AbstractList$ListItr.add:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "remove", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove(&self) -> Result<()> {
            let this = self;
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(())
        }

        #[java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE;>;)V")]
        pub fn forEachRemaining(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/AbstractList$ListItr.forEachRemaining:(Ljava/util/function/Consumer;)V")
        }
    }
}
