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

impl From<StreamOpFlag> for Enum<Object> {
    fn from(v: StreamOpFlag) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/StreamOpFlag"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/util/stream/StreamOpFlag;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "StreamOpFlag.java"]
    #[inner_classes     = "java/util/stream/StreamOpFlag$MaskBuilder:java/util/stream/StreamOpFlag:MaskBuilder:10;java/util/stream/StreamOpFlag$Type:java/util/stream/StreamOpFlag:Type:16408"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/util/stream/StreamOpFlag"]

    pub struct StreamOpFlag {
        #[cfg_attr(any(), java_field(name = "maskTable", descriptor = "Ljava/util/Map;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/Map<Ljava/util/stream/StreamOpFlag$Type;Ljava/lang/Integer;>;"))]
        pub maskTable: Object,
        #[cfg_attr(any(), java_field(name = "bitPosition", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub bitPosition: i32,
        #[cfg_attr(any(), java_field(name = "set", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub set: i32,
        #[cfg_attr(any(), java_field(name = "clear", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub clear: i32,
        #[cfg_attr(any(), java_field(name = "preserve", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub preserve: i32,
    }

    impl StreamOpFlag {
        #[cfg_attr(any(), java_field(name = "DISTINCT", descriptor = "Ljava/util/stream/StreamOpFlag;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DISTINCT:Ljava/util/stream/StreamOpFlag;
        pub fn DISTINCT() -> StreamOpFlag {
            panic!("stub: java/util/stream/StreamOpFlag.DISTINCT:Ljava/util/stream/StreamOpFlag;")
        }

        #[cfg_attr(any(), java_field(name = "SORTED", descriptor = "Ljava/util/stream/StreamOpFlag;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SORTED:Ljava/util/stream/StreamOpFlag;
        pub fn SORTED() -> StreamOpFlag {
            panic!("stub: java/util/stream/StreamOpFlag.SORTED:Ljava/util/stream/StreamOpFlag;")
        }

        #[cfg_attr(any(), java_field(name = "ORDERED", descriptor = "Ljava/util/stream/StreamOpFlag;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ORDERED:Ljava/util/stream/StreamOpFlag;
        pub fn ORDERED() -> StreamOpFlag {
            panic!("stub: java/util/stream/StreamOpFlag.ORDERED:Ljava/util/stream/StreamOpFlag;")
        }

        #[cfg_attr(any(), java_field(name = "SIZED", descriptor = "Ljava/util/stream/StreamOpFlag;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SIZED:Ljava/util/stream/StreamOpFlag;
        pub fn SIZED() -> StreamOpFlag {
            panic!("stub: java/util/stream/StreamOpFlag.SIZED:Ljava/util/stream/StreamOpFlag;")
        }

        #[cfg_attr(any(), java_field(name = "SHORT_CIRCUIT", descriptor = "Ljava/util/stream/StreamOpFlag;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SHORT_CIRCUIT:Ljava/util/stream/StreamOpFlag;
        pub fn SHORT_CIRCUIT() -> StreamOpFlag {
            panic!("stub: java/util/stream/StreamOpFlag.SHORT_CIRCUIT:Ljava/util/stream/StreamOpFlag;")
        }

        #[cfg_attr(any(), java_field(name = "SIZE_ADJUSTING", descriptor = "Ljava/util/stream/StreamOpFlag;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SIZE_ADJUSTING:Ljava/util/stream/StreamOpFlag;
        pub fn SIZE_ADJUSTING() -> StreamOpFlag {
            panic!("stub: java/util/stream/StreamOpFlag.SIZE_ADJUSTING:Ljava/util/stream/StreamOpFlag;")
        }

        #[cfg_attr(any(), java_field(name = "SET_BITS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: SET_BITS:I
        pub fn SET_BITS() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "CLEAR_BITS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: CLEAR_BITS:I
        pub fn CLEAR_BITS() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "PRESERVE_BITS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: PRESERVE_BITS:I
        pub fn PRESERVE_BITS() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "SPLITERATOR_CHARACTERISTICS_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: SPLITERATOR_CHARACTERISTICS_MASK:I
        pub fn SPLITERATOR_CHARACTERISTICS_MASK() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.SPLITERATOR_CHARACTERISTICS_MASK:I")
        }

        #[cfg_attr(any(), java_field(name = "STREAM_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: STREAM_MASK:I
        pub fn STREAM_MASK() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.STREAM_MASK:I")
        }

        #[cfg_attr(any(), java_field(name = "OP_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: OP_MASK:I
        pub fn OP_MASK() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.OP_MASK:I")
        }

        #[cfg_attr(any(), java_field(name = "TERMINAL_OP_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: TERMINAL_OP_MASK:I
        pub fn TERMINAL_OP_MASK() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.TERMINAL_OP_MASK:I")
        }

        #[cfg_attr(any(), java_field(name = "UPSTREAM_TERMINAL_OP_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: UPSTREAM_TERMINAL_OP_MASK:I
        pub fn UPSTREAM_TERMINAL_OP_MASK() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.UPSTREAM_TERMINAL_OP_MASK:I")
        }

        #[cfg_attr(any(), java_field(name = "FLAG_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: FLAG_MASK:I
        pub fn FLAG_MASK() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.FLAG_MASK:I")
        }

        #[cfg_attr(any(), java_field(name = "FLAG_MASK_IS", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: FLAG_MASK_IS:I
        pub fn FLAG_MASK_IS() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.FLAG_MASK_IS:I")
        }

        #[cfg_attr(any(), java_field(name = "FLAG_MASK_NOT", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: FLAG_MASK_NOT:I
        pub fn FLAG_MASK_NOT() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.FLAG_MASK_NOT:I")
        }

        #[cfg_attr(any(), java_field(name = "INITIAL_OPS_VALUE", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: INITIAL_OPS_VALUE:I
        pub fn INITIAL_OPS_VALUE() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.INITIAL_OPS_VALUE:I")
        }

        #[cfg_attr(any(), java_field(name = "IS_DISTINCT", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: IS_DISTINCT:I
        pub fn IS_DISTINCT() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.IS_DISTINCT:I")
        }

        #[cfg_attr(any(), java_field(name = "NOT_DISTINCT", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: NOT_DISTINCT:I
        pub fn NOT_DISTINCT() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.NOT_DISTINCT:I")
        }

        #[cfg_attr(any(), java_field(name = "IS_SORTED", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: IS_SORTED:I
        pub fn IS_SORTED() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.IS_SORTED:I")
        }

        #[cfg_attr(any(), java_field(name = "NOT_SORTED", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: NOT_SORTED:I
        pub fn NOT_SORTED() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.NOT_SORTED:I")
        }

        #[cfg_attr(any(), java_field(name = "IS_ORDERED", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: IS_ORDERED:I
        pub fn IS_ORDERED() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.IS_ORDERED:I")
        }

        #[cfg_attr(any(), java_field(name = "NOT_ORDERED", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: NOT_ORDERED:I
        pub fn NOT_ORDERED() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.NOT_ORDERED:I")
        }

        #[cfg_attr(any(), java_field(name = "IS_SIZED", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: IS_SIZED:I
        pub fn IS_SIZED() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.IS_SIZED:I")
        }

        #[cfg_attr(any(), java_field(name = "NOT_SIZED", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: NOT_SIZED:I
        pub fn NOT_SIZED() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.NOT_SIZED:I")
        }

        #[cfg_attr(any(), java_field(name = "IS_SHORT_CIRCUIT", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: IS_SHORT_CIRCUIT:I
        pub fn IS_SHORT_CIRCUIT() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.IS_SHORT_CIRCUIT:I")
        }

        #[cfg_attr(any(), java_field(name = "IS_SIZE_ADJUSTING", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: IS_SIZE_ADJUSTING:I
        pub fn IS_SIZE_ADJUSTING() -> i32 {
            panic!("stub: java/util/stream/StreamOpFlag.IS_SIZE_ADJUSTING:I")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/util/stream/StreamOpFlag;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/util/stream/StreamOpFlag;
        pub fn _VALUES() -> Rc<RefCell<Vec<StreamOpFlag>>> {
            panic!("stub: java/util/stream/StreamOpFlag.$VALUES:[Ljava/util/stream/StreamOpFlag;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/util/stream/StreamOpFlag;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<StreamOpFlag>>>> {
            panic!("stub: java/util/stream/StreamOpFlag.values:()[Ljava/util/stream/StreamOpFlag;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/util/stream/StreamOpFlag;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(name: String) -> Result<StreamOpFlag> {
            panic!("stub: java/util/stream/StreamOpFlag.valueOf:(Ljava/lang/String;)Ljava/util/stream/StreamOpFlag;")
        }

        #[java_method(name = "set", descriptor = "(Ljava/util/stream/StreamOpFlag$Type;)Ljava/util/stream/StreamOpFlag$MaskBuilder;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_stream(t: Object) -> Result<Object> {
            panic!("stub: java/util/stream/StreamOpFlag.set:(Ljava/util/stream/StreamOpFlag$Type;)Ljava/util/stream/StreamOpFlag$MaskBuilder;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;IILjava/util/stream/StreamOpFlag$MaskBuilder;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/stream/StreamOpFlag$MaskBuilder;)V", method_parameters = ":4096;:4096;:0;:0")]
        pub fn new(arg0: String, arg1: i32, position: i32, maskBuilder: Object) -> Result<Self> {
            panic!("stub: java/util/stream/StreamOpFlag.<init>:(Ljava/lang/String;IILjava/util/stream/StreamOpFlag$MaskBuilder;)V")
        }

        #[java_method(name = "set", descriptor = "()I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set(&self) -> Result<i32> {
            panic!("stub: java/util/stream/StreamOpFlag.set:()I")
        }

        #[java_method(name = "clear", descriptor = "()I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<i32> {
            panic!("stub: java/util/stream/StreamOpFlag.clear:()I")
        }

        #[java_method(name = "isStreamFlag", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isStreamFlag(&self) -> Result<bool> {
            panic!("stub: java/util/stream/StreamOpFlag.isStreamFlag:()Z")
        }

        #[java_method(name = "isKnown", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isKnown(&self, flags: i32) -> Result<bool> {
            panic!("stub: java/util/stream/StreamOpFlag.isKnown:(I)Z")
        }

        #[java_method(name = "isCleared", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCleared(&self, flags: i32) -> Result<bool> {
            panic!("stub: java/util/stream/StreamOpFlag.isCleared:(I)Z")
        }

        #[java_method(name = "isPreserved", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPreserved(&self, flags: i32) -> Result<bool> {
            panic!("stub: java/util/stream/StreamOpFlag.isPreserved:(I)Z")
        }

        #[java_method(name = "canSet", descriptor = "(Ljava/util/stream/StreamOpFlag$Type;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn canSet(&self, t: Object) -> Result<bool> {
            panic!("stub: java/util/stream/StreamOpFlag.canSet:(Ljava/util/stream/StreamOpFlag$Type;)Z")
        }

        #[java_method(name = "createMask", descriptor = "(Ljava/util/stream/StreamOpFlag$Type;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createMask(t: Object) -> Result<i32> {
            panic!("stub: java/util/stream/StreamOpFlag.createMask:(Ljava/util/stream/StreamOpFlag$Type;)I")
        }

        #[java_method(name = "createFlagMask", descriptor = "()I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createFlagMask() -> Result<i32> {
            panic!("stub: java/util/stream/StreamOpFlag.createFlagMask:()I")
        }

        #[java_method(name = "getMask", descriptor = "(I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMask(mut flags: i32) -> Result<i32> {
            Ok((if (flags==0) { StreamOpFlag::FLAG_MASK() } else { (((flags|((StreamOpFlag::FLAG_MASK_IS()&flags)<<(1i32&0x1f)))|((StreamOpFlag::FLAG_MASK_NOT()&flags)>>((1i32&0x1f))))^-1i32) }))
        }

        #[java_method(name = "combineOpFlags", descriptor = "(II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn combineOpFlags(mut newStreamOrOpFlags: i32, mut prevCombOpFlags: i32) -> Result<i32> {
            let _t0: i32 = StreamOpFlag::getMask(newStreamOrOpFlags)?;
            Ok(((prevCombOpFlags&_t0)|newStreamOrOpFlags))
        }

        #[java_method(name = "toStreamFlags", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toStreamFlags(combOpFlags: i32) -> Result<i32> {
            panic!("stub: java/util/stream/StreamOpFlag.toStreamFlags:(I)I")
        }

        #[java_method(name = "toCharacteristics", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toCharacteristics(streamFlags: i32) -> Result<i32> {
            panic!("stub: java/util/stream/StreamOpFlag.toCharacteristics:(I)I")
        }

        #[java_method(name = "fromCharacteristics", descriptor = "(Ljava/util/Spliterator;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<*>;)I")]
        // java: fromCharacteristics(Ljava/util/Spliterator;)I
        pub fn fromCharacteristics_splite(mut spliterator: Object) -> Result<i32> {
            let _vdispatch0: i32 = if let Some(_d) = spliterator.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef_Dropping<Object>>() { _d.characteristics()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef_Taking<Object>>() { _d.characteristics()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef<Object>>() { _d.characteristics()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<Stream_1>() { _d.characteristics()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<StreamSpliterators_InfiniteSupplyingSpliterator_OfRef<Object>>() { _d.characteristics()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<Spliterators_ArraySpliterator<Object>>() { _d.characteristics()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator<Object, Object>>() { _d.characteristics()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<Spliterators_AbstractSpliterator<Object>>() { _d.characteristics()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<StreamSpliterators_InfiniteSupplyingSpliterator<Object>>() { _d.characteristics()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<Object>() { _d.characteristics()? } else if let Some(__f) = spliterator.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let mut characteristics: i32 = _vdispatch0;
            let _vdispatch1: Object = if let Some(_d) = spliterator.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef_Dropping<Object>>() { _d.getComparator()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef_Taking<Object>>() { _d.getComparator()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef<Object>>() { _d.getComparator()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<Stream_1>() { _d.getComparator()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<StreamSpliterators_InfiniteSupplyingSpliterator_OfRef<Object>>() { _d.getComparator()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<Spliterators_ArraySpliterator<Object>>() { _d.getComparator()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator<Object, Object>>() { _d.getComparator()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<Spliterators_AbstractSpliterator<Object>>() { _d.getComparator()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<StreamSpliterators_InfiniteSupplyingSpliterator<Object>>() { _d.getComparator()? } else if let Some(_d) = spliterator.0.as_any().downcast_ref::<Object>() { _d.getComparator()? } else if let Some(__f) = spliterator.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            if !_is_jnull(&_vdispatch1) {
                return Ok(((characteristics&StreamOpFlag::SPLITERATOR_CHARACTERISTICS_MASK())&-5i32));
            }
            Ok((characteristics&StreamOpFlag::SPLITERATOR_CHARACTERISTICS_MASK()))
        }

        #[java_method(name = "fromCharacteristics", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fromCharacteristics_i(characteristics: i32) -> Result<i32> {
            panic!("stub: java/util/stream/StreamOpFlag.fromCharacteristics:(I)I")
        }
    }
}
