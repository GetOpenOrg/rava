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

impl From<ClassRepository> for GenericDeclRepository<Object> {
    fn from(v: ClassRepository) -> GenericDeclRepository<Object> { v.__into_super() }
}

impl From<ClassRepository> for AbstractRepository<Object> {
    fn from(v: ClassRepository) -> AbstractRepository<Object> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/reflect/generics/repository/ClassRepository"]
    #[super_class       = "sun/reflect/generics/repository/GenericDeclRepository"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "Lsun/reflect/generics/repository/GenericDeclRepository<Lsun/reflect/generics/tree/ClassSignature;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ClassRepository.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "GenericDeclRepository<Object>"]
    #[superclass_fields(factory: Object, tree: Object, typeParameters: Rc<RefCell<Vec<Object>>>)]
    #[all_supertypes    = "java/lang/Object;sun/reflect/generics/repository/AbstractRepository;sun/reflect/generics/repository/ClassRepository;sun/reflect/generics/repository/GenericDeclRepository"]

    pub struct ClassRepository {
        #[cfg_attr(any(), java_field(name = "superclass", descriptor = "Ljava/lang/reflect/Type;", access = "private", modifiers = "volatile", is_static = false))]
        pub superclass: Object,
        #[cfg_attr(any(), java_field(name = "superInterfaces", descriptor = "[Ljava/lang/reflect/Type;", access = "private", modifiers = "volatile", is_static = false))]
        pub superInterfaces: Rc<RefCell<Vec<Object>>>,
    }

    impl ClassRepository {
        #[cfg_attr(any(), java_field(name = "NONE", descriptor = "Lsun/reflect/generics/repository/ClassRepository;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NONE:Lsun/reflect/generics/repository/ClassRepository;
        pub fn NONE() -> ClassRepository {
            panic!("stub: sun/reflect/generics/repository/ClassRepository.NONE:Lsun/reflect/generics/repository/ClassRepository;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(rawSig: String, f: Object) -> Result<Self> {
            panic!("stub: sun/reflect/generics/repository/ClassRepository.<init>:(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/String;)Lsun/reflect/generics/tree/ClassSignature;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse(&self, s: String) -> Result<Object> {
            panic!("stub: sun/reflect/generics/repository/ClassRepository.parse:(Ljava/lang/String;)Lsun/reflect/generics/tree/ClassSignature;")
        }

        #[java_method(name = "make", descriptor = "(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)Lsun/reflect/generics/repository/ClassRepository;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn make(rawSig: String, f: Object) -> Result<ClassRepository> {
            panic!("stub: sun/reflect/generics/repository/ClassRepository.make:(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)Lsun/reflect/generics/repository/ClassRepository;")
        }

        #[java_method(name = "getSuperclass", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSuperclass(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/repository/ClassRepository.getSuperclass:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "getSuperInterfaces", descriptor = "()[Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSuperInterfaces(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: sun/reflect/generics/repository/ClassRepository.getSuperInterfaces:()[Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "computeSuperclass", descriptor = "()Ljava/lang/reflect/Type;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeSuperclass(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/repository/ClassRepository.computeSuperclass:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "computeSuperInterfaces", descriptor = "()[Ljava/lang/reflect/Type;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeSuperInterfaces(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: sun/reflect/generics/repository/ClassRepository.computeSuperInterfaces:()[Ljava/lang/reflect/Type;")
        }
    }
}
