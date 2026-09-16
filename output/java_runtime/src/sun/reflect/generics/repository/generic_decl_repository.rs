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

impl<S: Clone + Default + 'static> From<GenericDeclRepository<S>> for AbstractRepository<S> {
    fn from(v: GenericDeclRepository<S>) -> AbstractRepository<S> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/reflect/generics/repository/GenericDeclRepository"]
    #[super_class       = "sun/reflect/generics/repository/AbstractRepository"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<S::Lsun/reflect/generics/tree/Signature;>Lsun/reflect/generics/repository/AbstractRepository<TS;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "GenericDeclRepository.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractRepository<S>"]
    #[superclass_fields(factory: Object, tree: Object)]
    #[all_supertypes    = "java/lang/Object;sun/reflect/generics/repository/AbstractRepository;sun/reflect/generics/repository/GenericDeclRepository"]

    pub struct GenericDeclRepository<S: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "typeParameters", descriptor = "[Ljava/lang/reflect/TypeVariable;", access = "private", modifiers = "volatile", is_static = false, generic_signature = "[Ljava/lang/reflect/TypeVariable<*>;"))]
        pub typeParameters: Rc<RefCell<Vec<Object>>>,
    }

    impl<S> GenericDeclRepository<S> {
        #[cfg_attr(any(), java_field(name = "EMPTY_TYPE_VARS", descriptor = "[Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "static final", is_static = true, generic_signature = "[Ljava/lang/reflect/TypeVariable<*>;"))]
        // static field: EMPTY_TYPE_VARS:[Ljava/lang/reflect/TypeVariable;
        pub fn EMPTY_TYPE_VARS() -> Rc<RefCell<Vec<Object>>> {
            Rc::new(RefCell::new(Vec::new()))
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(rawSig: String, f: Object) -> Result<Self> {
            panic!("stub: sun/reflect/generics/repository/GenericDeclRepository.<init>:(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V")
        }

        #[java_method(name = "getTypeParameters", descriptor = "()[Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/reflect/TypeVariable<*>;")]
        pub fn getTypeParameters(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: sun/reflect/generics/repository/GenericDeclRepository.getTypeParameters:()[Ljava/lang/reflect/TypeVariable;")
        }

        #[java_method(name = "computeTypeParameters", descriptor = "()[Ljava/lang/reflect/TypeVariable;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/reflect/TypeVariable<*>;")]
        pub fn computeTypeParameters(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: sun/reflect/generics/repository/GenericDeclRepository.computeTypeParameters:()[Ljava/lang/reflect/TypeVariable;")
        }
    }
}
