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
    #[binary_name       = "java/util/stream/MatchOps"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "MatchOps.java"]
    #[inner_classes     = "java/util/stream/MatchOps$MatchOp:java/util/stream/MatchOps:MatchOp:26;java/util/stream/MatchOps$MatchKind:java/util/stream/MatchOps:MatchKind:16408;java/util/stream/MatchOps$4MatchSink::MatchSink:0;java/util/stream/MatchOps$3MatchSink::MatchSink:0;java/util/stream/MatchOps$2MatchSink::MatchSink:0;java/util/stream/MatchOps$1MatchSink::MatchSink:0;java/util/stream/MatchOps$MatchTask:java/util/stream/MatchOps:MatchTask:26;java/util/stream/MatchOps$BooleanTerminalSink:java/util/stream/MatchOps:BooleanTerminalSink:1034;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/MatchOps"]

    pub struct MatchOps;

    impl MatchOps {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/MatchOps.<init>:()V")
        }

        #[java_method(name = "makeRef", descriptor = "(Ljava/util/function/Predicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/Predicate<-TT;>;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp<TT;Ljava/lang/Boolean;>;")]
        pub fn makeRef(mut predicate: Object, mut matchKind: MatchOps_MatchKind) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&predicate))?;
            let _t1: Object = Objects::requireNonNull_obj(Object::from_any(matchKind.clone()))?;
            let __lam_cap21_0 = predicate;
            let __lam_cap21_1 = matchKind;
            let __lam_21: std::rc::Rc<dyn Fn() -> crate::error::Result<Object>> = std::rc::Rc::new(move || -> crate::error::Result<Object> { MatchOps::lambda_makeRef_0(__lam_cap21_0.clone(), __lam_cap21_1.clone()) });
            Ok(Object::from_any(MatchOps_MatchOp::<Object>::new(Clone::clone(&StreamShape::REFERENCE()), Clone::clone(&matchKind), Clone::clone(&Object::from_any(__lam_21)))?.clone()))
        }

        #[java_method(name = "makeInt", descriptor = "(Ljava/util/function/IntPredicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/IntPredicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp<Ljava/lang/Integer;Ljava/lang/Boolean;>;")]
        pub fn makeInt(predicate: Object, matchKind: MatchOps_MatchKind) -> Result<Object> {
            panic!("stub: java/util/stream/MatchOps.makeInt:(Ljava/util/function/IntPredicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeLong", descriptor = "(Ljava/util/function/LongPredicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/LongPredicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp<Ljava/lang/Long;Ljava/lang/Boolean;>;")]
        pub fn makeLong(predicate: Object, matchKind: MatchOps_MatchKind) -> Result<Object> {
            panic!("stub: java/util/stream/MatchOps.makeLong:(Ljava/util/function/LongPredicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeDouble", descriptor = "(Ljava/util/function/DoublePredicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/DoublePredicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp<Ljava/lang/Double;Ljava/lang/Boolean;>;")]
        pub fn makeDouble(predicate: Object, matchKind: MatchOps_MatchKind) -> Result<Object> {
            panic!("stub: java/util/stream/MatchOps.makeDouble:(Ljava/util/function/DoublePredicate;Ljava/util/stream/MatchOps$MatchKind;)Ljava/util/stream/TerminalOp;")
        }
    }
}
