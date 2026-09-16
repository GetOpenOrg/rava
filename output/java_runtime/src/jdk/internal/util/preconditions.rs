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
use crate::jdk::internal::util::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/util/Preconditions"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Preconditions.java"]
    #[inner_classes     = "jdk/internal/util/Preconditions$4:::0;jdk/internal/util/Preconditions$1:::0;jdk/internal/util/Preconditions$2:::0;jdk/internal/util/Preconditions$3:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/util/Preconditions"]

    pub struct Preconditions;

    impl Preconditions {
        #[cfg_attr(any(), java_field(name = "SIOOBE_FORMATTER", descriptor = "Ljava/util/function/BiFunction;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;Ljava/lang/StringIndexOutOfBoundsException;>;"))]
        // static field: SIOOBE_FORMATTER:Ljava/util/function/BiFunction;
        pub fn SIOOBE_FORMATTER() -> Object {
            panic!("stub: jdk/internal/util/Preconditions.SIOOBE_FORMATTER:Ljava/util/function/BiFunction;")
        }

        #[cfg_attr(any(), java_field(name = "AIOOBE_FORMATTER", descriptor = "Ljava/util/function/BiFunction;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;Ljava/lang/ArrayIndexOutOfBoundsException;>;"))]
        // static field: AIOOBE_FORMATTER:Ljava/util/function/BiFunction;
        pub fn AIOOBE_FORMATTER() -> Object {
            panic!("stub: jdk/internal/util/Preconditions.AIOOBE_FORMATTER:Ljava/util/function/BiFunction;")
        }

        #[cfg_attr(any(), java_field(name = "IOOBE_FORMATTER", descriptor = "Ljava/util/function/BiFunction;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;Ljava/lang/IndexOutOfBoundsException;>;"))]
        // static field: IOOBE_FORMATTER:Ljava/util/function/BiFunction;
        pub fn IOOBE_FORMATTER() -> Object {
            panic!("stub: jdk/internal/util/Preconditions.IOOBE_FORMATTER:Ljava/util/function/BiFunction;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/util/Preconditions.<init>:()V")
        }

        #[java_method(name = "outOfBounds", descriptor = "(Ljava/util/function/BiFunction;Ljava/lang/String;[Ljava/lang/Number;)Ljava/lang/RuntimeException;", access = "private", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;+Ljava/lang/RuntimeException;>;Ljava/lang/String;[Ljava/lang/Number;)Ljava/lang/RuntimeException;")]
        pub fn outOfBounds(oobef: Object, checkKind: String, args: Rc<RefCell<Vec<Number>>>) -> Result<RuntimeException> {
            panic!("stub: jdk/internal/util/Preconditions.outOfBounds:(Ljava/util/function/BiFunction;Ljava/lang/String;[Ljava/lang/Number;)Ljava/lang/RuntimeException;")
        }

        #[java_method(name = "outOfBoundsCheckIndex", descriptor = "(Ljava/util/function/BiFunction;II)Ljava/lang/RuntimeException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;+Ljava/lang/RuntimeException;>;II)Ljava/lang/RuntimeException;")]
        pub fn outOfBoundsCheckIndex_bifunc_i_i(oobe: Object, index: i32, length: i32) -> Result<RuntimeException> {
            panic!("stub: jdk/internal/util/Preconditions.outOfBoundsCheckIndex:(Ljava/util/function/BiFunction;II)Ljava/lang/RuntimeException;")
        }

        #[java_method(name = "outOfBoundsCheckFromToIndex", descriptor = "(Ljava/util/function/BiFunction;III)Ljava/lang/RuntimeException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;+Ljava/lang/RuntimeException;>;III)Ljava/lang/RuntimeException;")]
        pub fn outOfBoundsCheckFromToIndex_bifunc_i_i_i(oobe: Object, fromIndex: i32, toIndex: i32, length: i32) -> Result<RuntimeException> {
            panic!("stub: jdk/internal/util/Preconditions.outOfBoundsCheckFromToIndex:(Ljava/util/function/BiFunction;III)Ljava/lang/RuntimeException;")
        }

        #[java_method(name = "outOfBoundsCheckFromIndexSize", descriptor = "(Ljava/util/function/BiFunction;III)Ljava/lang/RuntimeException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;+Ljava/lang/RuntimeException;>;III)Ljava/lang/RuntimeException;")]
        pub fn outOfBoundsCheckFromIndexSize_bifunc_i_i_i(oobe: Object, fromIndex: i32, size: i32, length: i32) -> Result<RuntimeException> {
            panic!("stub: jdk/internal/util/Preconditions.outOfBoundsCheckFromIndexSize:(Ljava/util/function/BiFunction;III)Ljava/lang/RuntimeException;")
        }

        #[java_method(name = "outOfBoundsCheckIndex", descriptor = "(Ljava/util/function/BiFunction;JJ)Ljava/lang/RuntimeException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;+Ljava/lang/RuntimeException;>;JJ)Ljava/lang/RuntimeException;")]
        pub fn outOfBoundsCheckIndex_bifunc_l_l(oobe: Object, index: i64, arg2: i64) -> Result<RuntimeException> {
            panic!("stub: jdk/internal/util/Preconditions.outOfBoundsCheckIndex:(Ljava/util/function/BiFunction;JJ)Ljava/lang/RuntimeException;")
        }

        #[java_method(name = "outOfBoundsCheckFromToIndex", descriptor = "(Ljava/util/function/BiFunction;JJJ)Ljava/lang/RuntimeException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;+Ljava/lang/RuntimeException;>;JJJ)Ljava/lang/RuntimeException;")]
        pub fn outOfBoundsCheckFromToIndex_bifunc_l_l_l(oobe: Object, fromIndex: i64, arg2: i64, toIndex: i64) -> Result<RuntimeException> {
            panic!("stub: jdk/internal/util/Preconditions.outOfBoundsCheckFromToIndex:(Ljava/util/function/BiFunction;JJJ)Ljava/lang/RuntimeException;")
        }

        #[java_method(name = "outOfBoundsCheckFromIndexSize", descriptor = "(Ljava/util/function/BiFunction;JJJ)Ljava/lang/RuntimeException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;+Ljava/lang/RuntimeException;>;JJJ)Ljava/lang/RuntimeException;")]
        pub fn outOfBoundsCheckFromIndexSize_bifunc_l_l_l(oobe: Object, fromIndex: i64, arg2: i64, size: i64) -> Result<RuntimeException> {
            panic!("stub: jdk/internal/util/Preconditions.outOfBoundsCheckFromIndexSize:(Ljava/util/function/BiFunction;JJJ)Ljava/lang/RuntimeException;")
        }

        #[java_method(name = "outOfBoundsExceptionFormatter", descriptor = "(Ljava/util/function/Function;)Ljava/util/function/BiFunction;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<X:Ljava/lang/RuntimeException;>(Ljava/util/function/Function<Ljava/lang/String;TX;>;)Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;TX;>;")]
        pub fn outOfBoundsExceptionFormatter(f: Object) -> Result<Object> {
            panic!("stub: jdk/internal/util/Preconditions.outOfBoundsExceptionFormatter:(Ljava/util/function/Function;)Ljava/util/function/BiFunction;")
        }

        #[java_method(name = "outOfBoundsMessage", descriptor = "(Ljava/lang/String;Ljava/util/List;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/util/List<+Ljava/lang/Number;>;)Ljava/lang/String;")]
        pub fn outOfBoundsMessage(checkKind: String, args: Object) -> Result<String> {
            panic!("stub: jdk/internal/util/Preconditions.outOfBoundsMessage:(Ljava/lang/String;Ljava/util/List;)Ljava/lang/String;")
        }
    }
}
