#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "sun/reflect/generics/repository/GenericDeclRepository",
    super_class       = "sun/reflect/generics/repository/AbstractRepository",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "<S::Lsun/reflect/generics/tree/Signature;>Lsun/reflect/generics/repository/AbstractRepository<TS;>;",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "GenericDeclRepository.java",
    all_supertypes    = "java/lang/Object;sun/reflect/generics/repository/AbstractRepository;sun/reflect/generics/repository/GenericDeclRepository",
)]
#[derive(Clone, Default, PartialEq)]
pub struct GenericDeclRepository<S: Clone + Default + 'static> {
    pub _super: AbstractRepository<S>,
    #[cfg_attr(any(), java_field(name = "typeParameters", descriptor = "[Ljava/lang/reflect/TypeVariable;", access = "private", modifiers = "volatile", is_static = false, generic_signature = "[Ljava/lang/reflect/TypeVariable<*>;"))]
    pub typeParameters: JField<Rc<RefCell<Vec<Object>>>>,
    pub _phantom: std::marker::PhantomData<S>,
}

impl<S: Clone + Default + 'static> GenericDeclRepository<S> {
    pub fn as_abstract_repository(&self) -> &AbstractRepository<S> { &self._super }
    pub fn into_abstract_repository(self) -> AbstractRepository<S> { self._super }
}

impl<S: Clone + Default + 'static> From<GenericDeclRepository<S>> for AbstractRepository<S> {
    fn from(v: GenericDeclRepository<S>) -> AbstractRepository<S> { v._super }
}

impl<S: Clone + Default + 'static> GenericDeclRepository<S> {
    #[cfg_attr(any(), java_field(name = "EMPTY_TYPE_VARS", descriptor = "[Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "static final", is_static = true, generic_signature = "[Ljava/lang/reflect/TypeVariable<*>;"))]
    // static field: EMPTY_TYPE_VARS:[Ljava/lang/reflect/TypeVariable;
    pub fn EMPTY_TYPE_VARS() -> Rc<RefCell<Vec<Object>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(rawSig: String, f: Object) -> Result<Self> {
        panic!("stub: sun/reflect/generics/repository/GenericDeclRepository.<init>:(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V")
    }

    #[cfg_attr(any(), java_method(name = "getTypeParameters", descriptor = "()[Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/reflect/TypeVariable<*>;"))]
    pub fn getTypeParameters(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: sun/reflect/generics/repository/GenericDeclRepository.getTypeParameters:()[Ljava/lang/reflect/TypeVariable;")
    }

    #[cfg_attr(any(), java_method(name = "computeTypeParameters", descriptor = "()[Ljava/lang/reflect/TypeVariable;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/reflect/TypeVariable<*>;"))]
    pub fn computeTypeParameters(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: sun/reflect/generics/repository/GenericDeclRepository.computeTypeParameters:()[Ljava/lang/reflect/TypeVariable;")
    }
}
