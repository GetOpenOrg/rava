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
    #[binary_name       = "java/util/regex/Pattern$BitClass"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/regex/Pattern$BmpCharPredicate"]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Pattern.java"]
    #[inner_classes     = "java/util/regex/Pattern$BitClass:java/util/regex/Pattern:BitClass:24;java/util/regex/Pattern$BmpCharPredicate:java/util/regex/Pattern:BmpCharPredicate:1544"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/regex/Pattern$BitClass;java/util/regex/Pattern$BmpCharPredicate;java/util/regex/Pattern$CharPredicate"]

    pub struct Pattern_BitClass {
        #[cfg_attr(any(), java_field(name = "bits", descriptor = "[Z", access = "package", modifiers = "final", is_static = false))]
        pub bits: Rc<RefCell<Vec<bool>>>,
    }

    impl Pattern_BitClass {
        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            let mut _arr0: Rc<RefCell<Vec<bool>>> = Rc::new(RefCell::new(vec![false; 256i32 as usize]));
            this.__set_bits(Clone::clone(&_arr0));
            Ok(this)
        }

        #[java_method(name = "add", descriptor = "(II)Ljava/util/regex/Pattern$BitClass;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add(&self, mut c: i32, mut flags: i32) -> Result<Pattern_BitClass> {
            let this = self;
            if c > 255i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: bool = ASCII::isAscii(c)?;
            if _t0 {
                let _t1: i32 = ASCII::toUpper(c)?;
                this.__get_bits().borrow_mut()[_t1 as usize] = ((1i32) as i8 != 0);
                let _t2: i32 = ASCII::toLower(c)?;
                this.__get_bits().borrow_mut()[_t2 as usize] = ((1i32) as i8 != 0);
            } else {
                if ((flags&64i32)!=0) {
                    let _t1: i32 = Character::toLowerCase_i(c)?;
                    this.__get_bits().borrow_mut()[_t1 as usize] = ((1i32) as i8 != 0);
                    let _t2: i32 = Character::toUpperCase_i(c)?;
                    this.__get_bits().borrow_mut()[_t2 as usize] = ((1i32) as i8 != 0);
                }
            }
            this.__get_bits().borrow_mut()[c as usize] = ((1i32) as i8 != 0);
            Ok(Clone::clone(this))
        }

        #[java_method(name = "is", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn is(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern$BitClass.is:(I)Z")
        }

        #[java_method(name = "and", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn and_patter(&self, p: Object) -> Result<Object> {
            panic!("stub: java/util/regex/Pattern$BitClass.and:(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;")
        }

        #[java_method(name = "union", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn union_patter(&self, p: Object) -> Result<Object> {
            panic!("stub: java/util/regex/Pattern$BitClass.union:(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;")
        }

        #[java_method(name = "union", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn union_patter_patter(&self, p1: Object, p2: Object) -> Result<Object> {
            panic!("stub: java/util/regex/Pattern$BitClass.union:(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;")
        }

        #[java_method(name = "negate", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn negate(&self) -> Result<Object> {
            let this = self;
            let _t0: Object = Pattern::negate(Object::from_any(Clone::clone(self)))?;
            Ok(_t0)
        }
    }
}
