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
    #[binary_name       = "java/util/Spliterators"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Spliterators.java"]
    #[inner_classes     = "java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/Spliterator$OfLong:java/util/Spliterator:OfLong:1545;java/util/Spliterator$OfDouble:java/util/Spliterator:OfDouble:1545;java/util/Spliterators$ArraySpliterator:java/util/Spliterators:ArraySpliterator:24;java/util/Spliterators$IntArraySpliterator:java/util/Spliterators:IntArraySpliterator:24;java/util/Spliterators$LongArraySpliterator:java/util/Spliterators:LongArraySpliterator:24;java/util/Spliterators$DoubleArraySpliterator:java/util/Spliterators:DoubleArraySpliterator:24;java/util/Spliterators$IteratorSpliterator:java/util/Spliterators:IteratorSpliterator:8;java/util/Spliterators$IntIteratorSpliterator:java/util/Spliterators:IntIteratorSpliterator:24;java/util/PrimitiveIterator$OfInt:java/util/PrimitiveIterator:OfInt:1545;java/util/Spliterators$LongIteratorSpliterator:java/util/Spliterators:LongIteratorSpliterator:24;java/util/PrimitiveIterator$OfLong:java/util/PrimitiveIterator:OfLong:1545;java/util/Spliterators$DoubleIteratorSpliterator:java/util/Spliterators:DoubleIteratorSpliterator:24;java/util/PrimitiveIterator$OfDouble:java/util/PrimitiveIterator:OfDouble:1545;java/util/Spliterators$1Adapter::Adapter:0;java/util/Spliterators$2Adapter::Adapter:0;java/util/Spliterators$3Adapter::Adapter:0;java/util/Spliterators$4Adapter::Adapter:0;java/util/Spliterators$EmptySpliterator:java/util/Spliterators:EmptySpliterator:1034;java/util/Spliterators$EmptySpliterator$OfRef:java/util/Spliterators$EmptySpliterator:OfRef:26;java/util/Spliterators$EmptySpliterator$OfInt:java/util/Spliterators$EmptySpliterator:OfInt:26;java/util/Spliterators$EmptySpliterator$OfLong:java/util/Spliterators$EmptySpliterator:OfLong:26;java/util/Spliterators$EmptySpliterator$OfDouble:java/util/Spliterators$EmptySpliterator:OfDouble:26;java/util/Spliterators$AbstractDoubleSpliterator:java/util/Spliterators:AbstractDoubleSpliterator:1033;java/util/Spliterators$AbstractLongSpliterator:java/util/Spliterators:AbstractLongSpliterator:1033;java/util/Spliterators$AbstractIntSpliterator:java/util/Spliterators:AbstractIntSpliterator:1033;java/util/Spliterators$AbstractSpliterator:java/util/Spliterators:AbstractSpliterator:1033;java/util/Spliterators$AbstractDoubleSpliterator$HoldingDoubleConsumer:java/util/Spliterators$AbstractDoubleSpliterator:HoldingDoubleConsumer:24;java/util/Spliterators$AbstractLongSpliterator$HoldingLongConsumer:java/util/Spliterators$AbstractLongSpliterator:HoldingLongConsumer:24;java/util/Spliterators$AbstractIntSpliterator$HoldingIntConsumer:java/util/Spliterators$AbstractIntSpliterator:HoldingIntConsumer:24;java/util/Spliterators$AbstractSpliterator$HoldingConsumer:java/util/Spliterators$AbstractSpliterator:HoldingConsumer:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Spliterators"]

    pub struct Spliterators;

    impl Spliterators {
        #[cfg_attr(any(), java_field(name = "EMPTY_SPLITERATOR", descriptor = "Ljava/util/Spliterator;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Spliterator<Ljava/lang/Object;>;"))]
        // static field: EMPTY_SPLITERATOR:Ljava/util/Spliterator;
        pub fn EMPTY_SPLITERATOR() -> Object {
            panic!("stub: java/util/Spliterators.EMPTY_SPLITERATOR:Ljava/util/Spliterator;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_INT_SPLITERATOR", descriptor = "Ljava/util/Spliterator$OfInt;", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY_INT_SPLITERATOR:Ljava/util/Spliterator$OfInt;
        pub fn EMPTY_INT_SPLITERATOR() -> Object {
            panic!("stub: java/util/Spliterators.EMPTY_INT_SPLITERATOR:Ljava/util/Spliterator$OfInt;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_LONG_SPLITERATOR", descriptor = "Ljava/util/Spliterator$OfLong;", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY_LONG_SPLITERATOR:Ljava/util/Spliterator$OfLong;
        pub fn EMPTY_LONG_SPLITERATOR() -> Object {
            panic!("stub: java/util/Spliterators.EMPTY_LONG_SPLITERATOR:Ljava/util/Spliterator$OfLong;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_DOUBLE_SPLITERATOR", descriptor = "Ljava/util/Spliterator$OfDouble;", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY_DOUBLE_SPLITERATOR:Ljava/util/Spliterator$OfDouble;
        pub fn EMPTY_DOUBLE_SPLITERATOR() -> Object {
            panic!("stub: java/util/Spliterators.EMPTY_DOUBLE_SPLITERATOR:Ljava/util/Spliterator$OfDouble;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Spliterators.<init>:()V")
        }

        #[java_method(name = "emptySpliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/Spliterator<TT;>;")]
        pub fn emptySpliterator() -> Result<Object> {
            panic!("stub: java/util/Spliterators.emptySpliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "emptyIntSpliterator", descriptor = "()Ljava/util/Spliterator$OfInt;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn emptyIntSpliterator() -> Result<Object> {
            Ok(Spliterators::EMPTY_INT_SPLITERATOR())
        }

        #[java_method(name = "emptyLongSpliterator", descriptor = "()Ljava/util/Spliterator$OfLong;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn emptyLongSpliterator() -> Result<Object> {
            panic!("stub: java/util/Spliterators.emptyLongSpliterator:()Ljava/util/Spliterator$OfLong;")
        }

        #[java_method(name = "emptyDoubleSpliterator", descriptor = "()Ljava/util/Spliterator$OfDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn emptyDoubleSpliterator() -> Result<Object> {
            panic!("stub: java/util/Spliterators.emptyDoubleSpliterator:()Ljava/util/Spliterator$OfDouble;")
        }

        #[java_method(name = "spliterator", descriptor = "([Ljava/lang/Object;I)Ljava/util/Spliterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([Ljava/lang/Object;I)Ljava/util/Spliterator<TT;>;")]
        pub fn spliterator_arr_obj_i(array: Rc<RefCell<Vec<Object>>>, additionalCharacteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:([Ljava/lang/Object;I)Ljava/util/Spliterator;")
        }

        #[java_method(name = "spliterator", descriptor = "([Ljava/lang/Object;III)Ljava/util/Spliterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([Ljava/lang/Object;III)Ljava/util/Spliterator<TT;>;")]
        // java: spliterator([Ljava/lang/Object;III)Ljava/util/Spliterator;
        pub fn spliterator_arr_obj_i_i_i(mut array: Rc<RefCell<Vec<Object>>>, mut fromIndex: i32, mut toIndex: i32, mut additionalCharacteristics: i32) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Object::from_any(array.clone()))?;
            Spliterators::checkFromToBounds(((_t0).downcast::<Rc<RefCell<Vec<Object>>>>().borrow().len() as i32), fromIndex, toIndex)?;
            Ok(Object::from_any(Spliterators_ArraySpliterator::<Object>::new_arr_obj_i_i_i(Clone::clone(&array), fromIndex, toIndex, additionalCharacteristics)?.clone()))
        }

        #[java_method(name = "spliterator", descriptor = "([II)Ljava/util/Spliterator$OfInt;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_i_i(array: Rc<RefCell<Vec<i32>>>, additionalCharacteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:([II)Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "spliterator", descriptor = "([IIII)Ljava/util/Spliterator$OfInt;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_i_i_i_i(array: Rc<RefCell<Vec<i32>>>, fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:([IIII)Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "spliterator", descriptor = "([JI)Ljava/util/Spliterator$OfLong;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_l_i(array: Rc<RefCell<Vec<i64>>>, additionalCharacteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:([JI)Ljava/util/Spliterator$OfLong;")
        }

        #[java_method(name = "spliterator", descriptor = "([JIII)Ljava/util/Spliterator$OfLong;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_l_i_i_i(array: Rc<RefCell<Vec<i64>>>, fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:([JIII)Ljava/util/Spliterator$OfLong;")
        }

        #[java_method(name = "spliterator", descriptor = "([DI)Ljava/util/Spliterator$OfDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_d_i(array: Rc<RefCell<Vec<f64>>>, additionalCharacteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:([DI)Ljava/util/Spliterator$OfDouble;")
        }

        #[java_method(name = "spliterator", descriptor = "([DIII)Ljava/util/Spliterator$OfDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_d_i_i_i(array: Rc<RefCell<Vec<f64>>>, fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:([DIII)Ljava/util/Spliterator$OfDouble;")
        }

        #[java_method(name = "checkFromToBounds", descriptor = "(III)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkFromToBounds(mut arrayLength: i32, mut origin: i32, mut fence: i32) -> Result<()> {
            if origin > fence {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("origin(")))?;
                let _t1 = _t0.append_i(origin)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(") > fence(")))?;
                let _t3 = _t2.append_i(fence)?;
                let _t4 = _t3.append_str(Clone::clone(&String::from(")")))?;
                let _t5 = _t4.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (origin<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if fence > arrayLength {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "spliterator", descriptor = "(Ljava/util/Collection;I)Ljava/util/Spliterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Collection<+TT;>;I)Ljava/util/Spliterator<TT;>;")]
        pub fn spliterator_coll_i(c: Object, characteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:(Ljava/util/Collection;I)Ljava/util/Spliterator;")
        }

        #[java_method(name = "spliterator", descriptor = "(Ljava/util/Iterator;JI)Ljava/util/Spliterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Iterator<+TT;>;JI)Ljava/util/Spliterator<TT;>;")]
        pub fn spliterator_iterat_l_i(iterator: Object, size: i64, arg2: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:(Ljava/util/Iterator;JI)Ljava/util/Spliterator;")
        }

        #[java_method(name = "spliteratorUnknownSize", descriptor = "(Ljava/util/Iterator;I)Ljava/util/Spliterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Iterator<+TT;>;I)Ljava/util/Spliterator<TT;>;")]
        pub fn spliteratorUnknownSize_iterat_i(iterator: Object, characteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliteratorUnknownSize:(Ljava/util/Iterator;I)Ljava/util/Spliterator;")
        }

        #[java_method(name = "spliterator", descriptor = "(Ljava/util/PrimitiveIterator$OfInt;JI)Ljava/util/Spliterator$OfInt;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_primit_l_i(iterator: Object, size: i64, arg2: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:(Ljava/util/PrimitiveIterator$OfInt;JI)Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "spliteratorUnknownSize", descriptor = "(Ljava/util/PrimitiveIterator$OfInt;I)Ljava/util/Spliterator$OfInt;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliteratorUnknownSize_primit_i(iterator: Object, characteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliteratorUnknownSize:(Ljava/util/PrimitiveIterator$OfInt;I)Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "spliterator", descriptor = "(Ljava/util/PrimitiveIterator$OfLong;JI)Ljava/util/Spliterator$OfLong;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_primit_l_i_1(iterator: Object, size: i64, arg2: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:(Ljava/util/PrimitiveIterator$OfLong;JI)Ljava/util/Spliterator$OfLong;")
        }

        #[java_method(name = "spliteratorUnknownSize", descriptor = "(Ljava/util/PrimitiveIterator$OfLong;I)Ljava/util/Spliterator$OfLong;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliteratorUnknownSize_primit_i_1(iterator: Object, characteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliteratorUnknownSize:(Ljava/util/PrimitiveIterator$OfLong;I)Ljava/util/Spliterator$OfLong;")
        }

        #[java_method(name = "spliterator", descriptor = "(Ljava/util/PrimitiveIterator$OfDouble;JI)Ljava/util/Spliterator$OfDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_primit_l_i_2(iterator: Object, size: i64, arg2: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliterator:(Ljava/util/PrimitiveIterator$OfDouble;JI)Ljava/util/Spliterator$OfDouble;")
        }

        #[java_method(name = "spliteratorUnknownSize", descriptor = "(Ljava/util/PrimitiveIterator$OfDouble;I)Ljava/util/Spliterator$OfDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliteratorUnknownSize_primit_i_2(iterator: Object, characteristics: i32) -> Result<Object> {
            panic!("stub: java/util/Spliterators.spliteratorUnknownSize:(Ljava/util/PrimitiveIterator$OfDouble;I)Ljava/util/Spliterator$OfDouble;")
        }

        #[java_method(name = "iterator", descriptor = "(Ljava/util/Spliterator;)Ljava/util/Iterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Spliterator<+TT;>;)Ljava/util/Iterator<TT;>;")]
        pub fn iterator_splite(spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/Spliterators.iterator:(Ljava/util/Spliterator;)Ljava/util/Iterator;")
        }

        #[java_method(name = "iterator", descriptor = "(Ljava/util/Spliterator$OfInt;)Ljava/util/PrimitiveIterator$OfInt;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn iterator_splite_1(spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/Spliterators.iterator:(Ljava/util/Spliterator$OfInt;)Ljava/util/PrimitiveIterator$OfInt;")
        }

        #[java_method(name = "iterator", descriptor = "(Ljava/util/Spliterator$OfLong;)Ljava/util/PrimitiveIterator$OfLong;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn iterator_splite_2(spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/Spliterators.iterator:(Ljava/util/Spliterator$OfLong;)Ljava/util/PrimitiveIterator$OfLong;")
        }

        #[java_method(name = "iterator", descriptor = "(Ljava/util/Spliterator$OfDouble;)Ljava/util/PrimitiveIterator$OfDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn iterator_splite_3(spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/Spliterators.iterator:(Ljava/util/Spliterator$OfDouble;)Ljava/util/PrimitiveIterator$OfDouble;")
        }
    }
}
