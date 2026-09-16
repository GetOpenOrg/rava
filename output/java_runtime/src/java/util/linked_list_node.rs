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
    #[binary_name       = "java/util/LinkedList$Node"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/lang/Object;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LinkedList.java"]
    #[inner_classes     = "java/util/LinkedList$Node:java/util/LinkedList:Node:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/LinkedList$Node"]

    pub struct LinkedList_Node<E: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "item", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TE;"))]
        pub item: E,
        #[cfg_attr(any(), java_field(name = "next", descriptor = "Ljava/util/LinkedList$Node;", is_static = false, generic_signature = "Ljava/util/LinkedList$Node<TE;>;"))]
        pub next: LinkedList_Node<E>,
        #[cfg_attr(any(), java_field(name = "prev", descriptor = "Ljava/util/LinkedList$Node;", is_static = false, generic_signature = "Ljava/util/LinkedList$Node<TE;>;"))]
        pub prev: LinkedList_Node<E>,
    }

    impl<E> LinkedList_Node<E> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/LinkedList$Node;Ljava/lang/Object;Ljava/util/LinkedList$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/LinkedList$Node<TE;>;TE;Ljava/util/LinkedList$Node<TE;>;)V")]
        pub fn new(mut prev: LinkedList_Node<E>, mut element: E, mut next: LinkedList_Node<E>) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_item(Clone::clone(&element));
            this.__set_next(Clone::clone(&next));
            this.__set_prev(Clone::clone(&prev));
            Ok(this)
        }
    }
}
