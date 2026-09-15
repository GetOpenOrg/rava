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
use crate::jdk::internal::util::Preconditions;

#[java_rta_macros::java_class(
    binary_name       = "java/util/Objects",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Objects.java",
    all_supertypes    = "java/lang/Object;java/util/Objects",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Objects;

impl Objects {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/util/Objects.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(a: Object, b: Object) -> Result<bool> {
        panic!("stub: java/util/Objects.equals:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "deepEquals", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn deepEquals(a: Object, b: Object) -> Result<bool> {
        panic!("stub: java/util/Objects.deepEquals:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(o: Object) -> Result<i32> {
        panic!("stub: java/util/Objects.hashCode:(Ljava/lang/Object;)I")
    }

    #[cfg_attr(any(), java_method(name = "hash", descriptor = "([Ljava/lang/Object;)I", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hash(values: Rc<RefCell<Vec<Object>>>) -> Result<i32> {
        panic!("stub: java/util/Objects.hash:([Ljava/lang/Object;)I")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString_obj(o: Object) -> Result<String> {
        panic!("stub: java/util/Objects.toString:(Ljava/lang/Object;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString_obj_str(o: Object, nullDefault: String) -> Result<String> {
        panic!("stub: java/util/Objects.toString:(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toIdentityString", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toIdentityString(o: Object) -> Result<String> {
        panic!("stub: java/util/Objects.toIdentityString:(Ljava/lang/Object;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "compare", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;TT;Ljava/util/Comparator<-TT;>;)I"))]
    pub fn compare(a: Object, b: Object, c: Object) -> Result<i32> {
        panic!("stub: java/util/Objects.compare:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I")
    }

    #[cfg_attr(any(), java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)TT;"))]
    pub fn requireNonNull_obj(obj: Object) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNull:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/lang/String;)TT;"))]
    pub fn requireNonNull_obj_str(obj: Object, message: String) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNull:(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "isNull", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isNull(obj: Object) -> Result<bool> {
        panic!("stub: java/util/Objects.isNull:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "nonNull", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nonNull(obj: Object) -> Result<bool> {
        panic!("stub: java/util/Objects.nonNull:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "requireNonNullElse", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;TT;)TT;"))]
    pub fn requireNonNullElse(obj: Object, defaultObj: Object) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNullElse:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "requireNonNullElseGet", descriptor = "(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/util/function/Supplier<+TT;>;)TT;"))]
    pub fn requireNonNullElseGet(obj: Object, supplier: Object) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNullElseGet:(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/util/function/Supplier<Ljava/lang/String;>;)TT;"))]
    pub fn requireNonNull_obj_suppli(obj: Object, messageSupplier: Object) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNull:(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "checkIndex", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: checkIndex(II)I
    pub fn checkIndex_i_i(mut index: i32, mut length: i32) -> Result<i32> {
        let _t0: i32 = Preconditions::checkIndex_i_i_bifunc(index, length, Default::default())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "checkFromToIndex", descriptor = "(III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkFromToIndex_i_i_i(fromIndex: i32, toIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: java/util/Objects.checkFromToIndex:(III)I")
    }

    #[cfg_attr(any(), java_method(name = "checkFromIndexSize", descriptor = "(III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkFromIndexSize_i_i_i(fromIndex: i32, size: i32, length: i32) -> Result<i32> {
        panic!("stub: java/util/Objects.checkFromIndexSize:(III)I")
    }

    #[cfg_attr(any(), java_method(name = "checkIndex", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkIndex_l_l(index: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/util/Objects.checkIndex:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "checkFromToIndex", descriptor = "(JJJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkFromToIndex_l_l_l(fromIndex: i64, arg1: i64, toIndex: i64) -> Result<i64> {
        panic!("stub: java/util/Objects.checkFromToIndex:(JJJ)J")
    }

    #[cfg_attr(any(), java_method(name = "checkFromIndexSize", descriptor = "(JJJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkFromIndexSize_l_l_l(fromIndex: i64, arg1: i64, size: i64) -> Result<i64> {
        panic!("stub: java/util/Objects.checkFromIndexSize:(JJJ)J")
    }
}
