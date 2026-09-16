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
    #[binary_name       = "java/nio/charset/CoderResult"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CoderResult.java"]
    #[inner_classes     = "java/nio/charset/CoderResult$Cache:java/nio/charset/CoderResult:Cache:26;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/nio/charset/CoderResult"]
    #[has_to_string_method = true]

    pub struct CoderResult {
        #[cfg_attr(any(), java_field(name = "type", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub type_: i32,
        #[cfg_attr(any(), java_field(name = "length", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub length: i32,
    }

    impl CoderResult {
        #[cfg_attr(any(), java_field(name = "CR_UNDERFLOW", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: CR_UNDERFLOW:I
        pub fn CR_UNDERFLOW() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "CR_OVERFLOW", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: CR_OVERFLOW:I
        pub fn CR_OVERFLOW() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "CR_ERROR_MIN", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: CR_ERROR_MIN:I
        pub fn CR_ERROR_MIN() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "CR_MALFORMED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: CR_MALFORMED:I
        pub fn CR_MALFORMED() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "CR_UNMAPPABLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: CR_UNMAPPABLE:I
        pub fn CR_UNMAPPABLE() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "names", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: names:[Ljava/lang/String;
        pub fn names() -> Rc<RefCell<Vec<String>>> {
            panic!("stub: java/nio/charset/CoderResult.names:[Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "UNDERFLOW", descriptor = "Ljava/nio/charset/CoderResult;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UNDERFLOW:Ljava/nio/charset/CoderResult;
        pub fn UNDERFLOW() -> CoderResult {
            panic!("stub: java/nio/charset/CoderResult.UNDERFLOW:Ljava/nio/charset/CoderResult;")
        }

        #[cfg_attr(any(), java_field(name = "OVERFLOW", descriptor = "Ljava/nio/charset/CoderResult;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OVERFLOW:Ljava/nio/charset/CoderResult;
        pub fn OVERFLOW() -> CoderResult {
            panic!("stub: java/nio/charset/CoderResult.OVERFLOW:Ljava/nio/charset/CoderResult;")
        }

        #[cfg_attr(any(), java_field(name = "malformed4", descriptor = "[Ljava/nio/charset/CoderResult;", access = "private", modifiers = "static final", is_static = true))]
        // static field: malformed4:[Ljava/nio/charset/CoderResult;
        pub fn malformed4() -> Rc<RefCell<Vec<CoderResult>>> {
            panic!("stub: java/nio/charset/CoderResult.malformed4:[Ljava/nio/charset/CoderResult;")
        }

        #[cfg_attr(any(), java_field(name = "unmappable4", descriptor = "[Ljava/nio/charset/CoderResult;", access = "private", modifiers = "static final", is_static = true))]
        // static field: unmappable4:[Ljava/nio/charset/CoderResult;
        pub fn unmappable4() -> Rc<RefCell<Vec<CoderResult>>> {
            panic!("stub: java/nio/charset/CoderResult.unmappable4:[Ljava/nio/charset/CoderResult;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(type_: i32, length: i32) -> Result<Self> {
            panic!("stub: java/nio/charset/CoderResult.<init>:(II)V")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            let mut nm = Clone::clone(&CoderResult::names().borrow()[this.__get_type_() as usize]);
            let _t0 = this.isError()?;
            let mut _merged6: String;
            if _t0 {
                let _t1 = StringBuilder::new()?.append_str(Clone::clone(&nm))?;
                let _t2 = _t1.append_str(Clone::clone(&String::from("[")))?;
                let _t3 = _t2.append_i(this.__get_length())?;
                let _t4 = _t3.append_str(Clone::clone(&String::from("]")))?;
                let _t5 = _t4.toString()?;
                _merged6 = _t5;
            } else {
                _merged6 = nm;
            }
            Ok(_merged6)
        }

        #[java_method(name = "isUnderflow", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnderflow(&self) -> Result<bool> {
            let this = self;
            Ok((this.__get_type_()==0))
        }

        #[java_method(name = "isOverflow", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOverflow(&self) -> Result<bool> {
            let this = self;
            Ok(this.__get_type_() == 1i32)
        }

        #[java_method(name = "isError", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isError(&self) -> Result<bool> {
            let this = self;
            Ok(this.__get_type_() >= 2i32)
        }

        #[java_method(name = "isMalformed", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMalformed(&self) -> Result<bool> {
            let this = self;
            Ok(this.__get_type_() == 2i32)
        }

        #[java_method(name = "isUnmappable", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnmappable(&self) -> Result<bool> {
            let this = self;
            Ok(this.__get_type_() == 3i32)
        }

        #[java_method(name = "length", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn length(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.isError()?;
            if !(_t0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(this.__get_length())
        }

        #[java_method(name = "malformedForLength", descriptor = "(I)Ljava/nio/charset/CoderResult;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn malformedForLength(mut length: i32) -> Result<CoderResult> {
            if (length<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if length <= 4i32 {
                return Ok(Clone::clone(&CoderResult::malformed4().borrow()[(length).wrapping_sub(1i32) as usize]));
            }
            let __lam_72: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { CoderResult::lambda_malformedForLength_0(_la0) });
            let _vdispatch0: Object = if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<Properties>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<Object>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(_d) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<Object>() { _d.computeIfAbsent(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else if let Some(__f) = CoderResult_Cache::INSTANCE().__get_malformed().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>>>() { (__f)(length.into(), Clone::clone(&Object::from_any(__lam_72)))? } else { Default::default() };
            Ok((_vdispatch0).downcast::<CoderResult>())
        }

        #[java_method(name = "unmappableForLength", descriptor = "(I)Ljava/nio/charset/CoderResult;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unmappableForLength(length: i32) -> Result<CoderResult> {
            panic!("stub: java/nio/charset/CoderResult.unmappableForLength:(I)Ljava/nio/charset/CoderResult;")
        }

        #[java_method(name = "throwException", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/nio/charset/CharacterCodingException")]
        pub fn throwException(&self) -> Result<()> {
            let this = self;
            let _switch_key = this.__get_type_();
            return Err(JvmError::Custom("athrow".to_owned()));
            return Err(JvmError::Custom("athrow".to_owned()));
            return Err(JvmError::Custom("athrow".to_owned()));
            return Err(JvmError::Custom("athrow".to_owned()));
            if !(CoderResult::_assertionsDisabled()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }
    }
}
