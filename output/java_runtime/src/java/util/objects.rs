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
use crate::jdk::internal::util::Preconditions;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Objects"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Objects.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Objects"]

    pub struct Objects;

    impl Objects {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Objects.<init>:()V")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(mut a: Object, mut b: Object) -> Result<bool> {
            let mut _merged1: bool;
            if !_is_jnull(&a) {
                let _vdispatch0: bool = if let Some(__f) = a.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&b))? } else { Default::default() };
                _merged1 = !(!(_vdispatch0));
            } else {
                _merged1 = (0i32 != 0);
            }
            Ok(_merged1)
        }

        #[java_method(name = "deepEquals", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn deepEquals(a: Object, b: Object) -> Result<bool> {
            panic!("stub: java/util/Objects.deepEquals:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(o: Object) -> Result<i32> {
            panic!("stub: java/util/Objects.hashCode:(Ljava/lang/Object;)I")
        }

        #[java_method(name = "hash", descriptor = "([Ljava/lang/Object;)I", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hash(mut values: Rc<RefCell<Vec<Object>>>) -> Result<i32> {
            let _t0: i32 = Arrays::hashCode_arr_obj(Clone::clone(&values))?;
            Ok(_t0)
        }

        #[java_method(name = "toString", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_obj(o: Object) -> Result<String> {
            panic!("stub: java/util/Objects.toString:(Ljava/lang/Object;)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_obj_str(o: Object, nullDefault: String) -> Result<String> {
            panic!("stub: java/util/Objects.toString:(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "toIdentityString", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toIdentityString(o: Object) -> Result<String> {
            panic!("stub: java/util/Objects.toIdentityString:(Ljava/lang/Object;)Ljava/lang/String;")
        }

        #[java_method(name = "compare", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;TT;Ljava/util/Comparator<-TT;>;)I")]
        pub fn compare(a: Object, b: Object, c: Object) -> Result<i32> {
            panic!("stub: java/util/Objects.compare:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I")
        }

        #[java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)TT;")]
        // java: requireNonNull(Ljava/lang/Object;)Ljava/lang/Object;
        pub fn requireNonNull_obj(mut obj: Object) -> Result<Object> {
            if _is_jnull(&obj) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(obj)
        }

        #[java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/lang/String;)TT;")]
        // java: requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;
        pub fn requireNonNull_obj_str(mut obj: Object, mut message: String) -> Result<Object> {
            if _is_jnull(&obj) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(obj)
        }

        #[java_method(name = "isNull", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNull(obj: Object) -> Result<bool> {
            panic!("stub: java/util/Objects.isNull:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "nonNull", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nonNull(obj: Object) -> Result<bool> {
            panic!("stub: java/util/Objects.nonNull:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "requireNonNullElse", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;TT;)TT;")]
        pub fn requireNonNullElse(mut obj: Object, mut defaultObj: Object) -> Result<Object> {
            let mut _merged1: Object;
            if !_is_jnull(&obj) {
                _merged1 = obj;
            } else {
                let _t0: Object = Objects::requireNonNull_obj_str(Clone::clone(&defaultObj), Clone::clone(&String::from("defaultObj")))?;
                _merged1 = _t0;
            }
            Ok(_merged1)
        }

        #[java_method(name = "requireNonNullElseGet", descriptor = "(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/util/function/Supplier<+TT;>;)TT;")]
        pub fn requireNonNullElseGet(obj: Object, supplier: Object) -> Result<Object> {
            panic!("stub: java/util/Objects.requireNonNullElseGet:(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;")
        }

        #[java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/util/function/Supplier<Ljava/lang/String;>;)TT;")]
        pub fn requireNonNull_obj_suppli(obj: Object, messageSupplier: Object) -> Result<Object> {
            panic!("stub: java/util/Objects.requireNonNull:(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;")
        }

        #[java_method(name = "checkIndex", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: checkIndex(II)I
        pub fn checkIndex_i_i(mut index: i32, mut length: i32) -> Result<i32> {
            let _t0: i32 = Preconditions::checkIndex_i_i_bifunc(index, length, Clone::clone(&Object::default()))?;
            Ok(_t0)
        }

        #[java_method(name = "checkFromToIndex", descriptor = "(III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: checkFromToIndex(III)I
        pub fn checkFromToIndex_i_i_i(mut fromIndex: i32, mut toIndex: i32, mut length: i32) -> Result<i32> {
            let _t0: i32 = Preconditions::checkFromToIndex_i_i_i_bifunc(fromIndex, toIndex, length, Clone::clone(&Object::default()))?;
            Ok(_t0)
        }

        #[java_method(name = "checkFromIndexSize", descriptor = "(III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: checkFromIndexSize(III)I
        pub fn checkFromIndexSize_i_i_i(mut fromIndex: i32, mut size: i32, mut length: i32) -> Result<i32> {
            let _t0: i32 = Preconditions::checkFromIndexSize_i_i_i_bifunc(fromIndex, size, length, Clone::clone(&Object::default()))?;
            Ok(_t0)
        }

        #[java_method(name = "checkIndex", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkIndex_l_l(index: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/util/Objects.checkIndex:(JJ)J")
        }

        #[java_method(name = "checkFromToIndex", descriptor = "(JJJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkFromToIndex_l_l_l(fromIndex: i64, arg1: i64, toIndex: i64) -> Result<i64> {
            panic!("stub: java/util/Objects.checkFromToIndex:(JJJ)J")
        }

        #[java_method(name = "checkFromIndexSize", descriptor = "(JJJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkFromIndexSize_l_l_l(fromIndex: i64, arg1: i64, size: i64) -> Result<i64> {
            panic!("stub: java/util/Objects.checkFromIndexSize:(JJJ)J")
        }
    }
}
