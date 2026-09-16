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
    #[binary_name       = "java/util/ImmutableCollections"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ImmutableCollections.java"]
    #[inner_classes     = "java/util/ImmutableCollections$List12:java/util/ImmutableCollections:List12:24;java/util/ImmutableCollections$ListN:java/util/ImmutableCollections:ListN:24;java/util/ImmutableCollections$SetN:java/util/ImmutableCollections:SetN:24;java/util/ImmutableCollections$MapN:java/util/ImmutableCollections:MapN:24;java/util/ImmutableCollections$Map1:java/util/ImmutableCollections:Map1:24;java/util/ImmutableCollections$AbstractImmutableMap:java/util/ImmutableCollections:AbstractImmutableMap:1032;java/util/ImmutableCollections$Set12:java/util/ImmutableCollections:Set12:24;java/util/ImmutableCollections$AbstractImmutableSet:java/util/ImmutableCollections:AbstractImmutableSet:1032;java/util/ImmutableCollections$SubList:java/util/ImmutableCollections:SubList:24;java/util/ImmutableCollections$ListItr:java/util/ImmutableCollections:ListItr:24;java/util/ImmutableCollections$AbstractImmutableList:java/util/ImmutableCollections:AbstractImmutableList:1032;java/util/ImmutableCollections$AbstractImmutableCollection:java/util/ImmutableCollections:AbstractImmutableCollection:1032;java/util/ImmutableCollections$Access:java/util/ImmutableCollections:Access:8;java/util/ImmutableCollections$MapN$MapNIterator:java/util/ImmutableCollections$MapN:MapNIterator:0;java/util/ImmutableCollections$MapN$1:::0;java/util/ImmutableCollections$SetN$SetNIterator:java/util/ImmutableCollections$SetN:SetNIterator:18;java/util/ImmutableCollections$Set12$1:::0;java/util/ImmutableCollections$Access$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/ImmutableCollections"]

    pub struct ImmutableCollections;

    impl ImmutableCollections {
        #[cfg_attr(any(), java_field(name = "SALT32L", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: SALT32L:J
        pub fn SALT32L() -> i64 {
            panic!("stub: java/util/ImmutableCollections.SALT32L:J")
        }

        #[cfg_attr(any(), java_field(name = "REVERSE", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
        // static field: REVERSE:Z
        pub fn REVERSE() -> bool {
            false
        }

        #[cfg_attr(any(), java_field(name = "archivedObjects", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "static", is_static = true))]
        // static field: archivedObjects:[Ljava/lang/Object;
        pub fn archivedObjects() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/util/ImmutableCollections.archivedObjects:[Ljava/lang/Object;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY:Ljava/lang/Object;
        pub fn EMPTY() -> Object {
            panic!("stub: java/util/ImmutableCollections.EMPTY:Ljava/lang/Object;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_LIST", descriptor = "Ljava/util/ImmutableCollections$ListN;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/ImmutableCollections$ListN<*>;"))]
        // static field: EMPTY_LIST:Ljava/util/ImmutableCollections$ListN;
        pub fn EMPTY_LIST() -> Object {
            panic!("stub: java/util/ImmutableCollections.EMPTY_LIST:Ljava/util/ImmutableCollections$ListN;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_LIST_NULLS", descriptor = "Ljava/util/ImmutableCollections$ListN;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/ImmutableCollections$ListN<*>;"))]
        // static field: EMPTY_LIST_NULLS:Ljava/util/ImmutableCollections$ListN;
        pub fn EMPTY_LIST_NULLS() -> Object {
            panic!("stub: java/util/ImmutableCollections.EMPTY_LIST_NULLS:Ljava/util/ImmutableCollections$ListN;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_SET", descriptor = "Ljava/util/ImmutableCollections$SetN;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/ImmutableCollections$SetN<*>;"))]
        // static field: EMPTY_SET:Ljava/util/ImmutableCollections$SetN;
        pub fn EMPTY_SET() -> Object {
            panic!("stub: java/util/ImmutableCollections.EMPTY_SET:Ljava/util/ImmutableCollections$SetN;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_MAP", descriptor = "Ljava/util/ImmutableCollections$MapN;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/ImmutableCollections$MapN<**>;"))]
        // static field: EMPTY_MAP:Ljava/util/ImmutableCollections$MapN;
        pub fn EMPTY_MAP() -> ImmutableCollections_MapN<Object, Object> {
            panic!("stub: java/util/ImmutableCollections.EMPTY_MAP:Ljava/util/ImmutableCollections$MapN;")
        }

        #[cfg_attr(any(), java_field(name = "EXPAND_FACTOR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: EXPAND_FACTOR:I
        pub fn EXPAND_FACTOR() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/ImmutableCollections.<init>:()V")
        }

        #[java_method(name = "uoe", descriptor = "()Ljava/lang/UnsupportedOperationException;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn uoe() -> Result<UnsupportedOperationException> {
            panic!("stub: java/util/ImmutableCollections.uoe:()Ljava/lang/UnsupportedOperationException;")
        }

        #[java_method(name = "listCopy", descriptor = "(Ljava/util/Collection;)Ljava/util/List;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/Collection<+TE;>;)Ljava/util/List<TE;>;")]
        pub fn listCopy(coll: Object) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections.listCopy:(Ljava/util/Collection;)Ljava/util/List;")
        }

        #[java_method(name = "listFromArray", descriptor = "([Ljava/lang/Object;)Ljava/util/List;", access = "package", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>([TE;)Ljava/util/List<TE;>;")]
        pub fn listFromArray(input: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections.listFromArray:([Ljava/lang/Object;)Ljava/util/List;")
        }

        #[java_method(name = "listFromTrustedArray", descriptor = "([Ljava/lang/Object;)Ljava/util/List;", access = "package", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>([Ljava/lang/Object;)Ljava/util/List<TE;>;")]
        pub fn listFromTrustedArray(input: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections.listFromTrustedArray:([Ljava/lang/Object;)Ljava/util/List;")
        }

        #[java_method(name = "listFromTrustedArrayNullsAllowed", descriptor = "([Ljava/lang/Object;)Ljava/util/List;", access = "package", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>([Ljava/lang/Object;)Ljava/util/List<TE;>;")]
        pub fn listFromTrustedArrayNullsAllowed(input: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections.listFromTrustedArrayNullsAllowed:([Ljava/lang/Object;)Ljava/util/List;")
        }
    }
}
