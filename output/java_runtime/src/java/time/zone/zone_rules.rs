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
    #[binary_name       = "java/time/zone/ZoneRules"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ZoneRules.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/time/zone/ZoneRules"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ZoneRules {
        #[cfg_attr(any(), java_field(name = "standardTransitions", descriptor = "[J", access = "private", modifiers = "final", is_static = false))]
        pub standardTransitions: Rc<RefCell<Vec<i64>>>,
        #[cfg_attr(any(), java_field(name = "standardOffsets", descriptor = "[Ljava/time/ZoneOffset;", access = "private", modifiers = "final", is_static = false))]
        pub standardOffsets: Rc<RefCell<Vec<ZoneOffset>>>,
        #[cfg_attr(any(), java_field(name = "savingsInstantTransitions", descriptor = "[J", access = "private", modifiers = "final", is_static = false))]
        pub savingsInstantTransitions: Rc<RefCell<Vec<i64>>>,
        #[cfg_attr(any(), java_field(name = "savingsLocalTransitions", descriptor = "[Ljava/time/LocalDateTime;", access = "private", modifiers = "final", is_static = false))]
        pub savingsLocalTransitions: Rc<RefCell<Vec<LocalDateTime>>>,
        #[cfg_attr(any(), java_field(name = "wallOffsets", descriptor = "[Ljava/time/ZoneOffset;", access = "private", modifiers = "final", is_static = false))]
        pub wallOffsets: Rc<RefCell<Vec<ZoneOffset>>>,
        #[cfg_attr(any(), java_field(name = "lastRules", descriptor = "[Ljava/time/zone/ZoneOffsetTransitionRule;", access = "private", modifiers = "final", is_static = false))]
        pub lastRules: Rc<RefCell<Vec<ZoneOffsetTransitionRule>>>,
        #[cfg_attr(any(), java_field(name = "lastRulesCache", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "final transient", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/lang/Integer;[Ljava/time/zone/ZoneOffsetTransition;>;"))]
        pub lastRulesCache: Object,
    }

    impl ZoneRules {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "3044319355680032515"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            3044319355680032515i64
        }

        #[cfg_attr(any(), java_field(name = "LAST_CACHED_YEAR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2100"))]
        // static field: LAST_CACHED_YEAR:I
        pub fn LAST_CACHED_YEAR() -> i32 {
            2100
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_LONG_ARRAY", descriptor = "[J", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY_LONG_ARRAY:[J
        pub fn EMPTY_LONG_ARRAY() -> Rc<RefCell<Vec<i64>>> {
            panic!("stub: java/time/zone/ZoneRules.EMPTY_LONG_ARRAY:[J")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_LASTRULES", descriptor = "[Ljava/time/zone/ZoneOffsetTransitionRule;", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY_LASTRULES:[Ljava/time/zone/ZoneOffsetTransitionRule;
        pub fn EMPTY_LASTRULES() -> Rc<RefCell<Vec<ZoneOffsetTransitionRule>>> {
            Rc::new(RefCell::new(Vec::new()))
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_LDT_ARRAY", descriptor = "[Ljava/time/LocalDateTime;", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY_LDT_ARRAY:[Ljava/time/LocalDateTime;
        pub fn EMPTY_LDT_ARRAY() -> Rc<RefCell<Vec<LocalDateTime>>> {
            Rc::new(RefCell::new(Vec::new()))
        }

        #[cfg_attr(any(), java_field(name = "DAYS_PER_CYCLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "146097"))]
        // static field: DAYS_PER_CYCLE:I
        pub fn DAYS_PER_CYCLE() -> i32 {
            146097
        }

        #[cfg_attr(any(), java_field(name = "DAYS_0000_TO_1970", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "719528"))]
        // static field: DAYS_0000_TO_1970:J
        pub fn DAYS_0000_TO_1970() -> i64 {
            719528i64
        }

        #[java_method(name = "of", descriptor = "(Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/util/List;Ljava/util/List;Ljava/util/List;)Ljava/time/zone/ZoneRules;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/util/List<Ljava/time/zone/ZoneOffsetTransition;>;Ljava/util/List<Ljava/time/zone/ZoneOffsetTransition;>;Ljava/util/List<Ljava/time/zone/ZoneOffsetTransitionRule;>;)Ljava/time/zone/ZoneRules;")]
        pub fn of_zoneof_zoneof_list_list_list(baseStandardOffset: ZoneOffset, baseWallOffset: ZoneOffset, standardOffsetTransitionList: Object, transitionList: Object, lastRules: Object) -> Result<ZoneRules> {
            panic!("stub: java/time/zone/ZoneRules.of:(Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/util/List;Ljava/util/List;Ljava/util/List;)Ljava/time/zone/ZoneRules;")
        }

        #[java_method(name = "of", descriptor = "(Ljava/time/ZoneOffset;)Ljava/time/zone/ZoneRules;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_zoneof(offset: ZoneOffset) -> Result<ZoneRules> {
            panic!("stub: java/time/zone/ZoneRules.of:(Ljava/time/ZoneOffset;)Ljava/time/zone/ZoneRules;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/util/List;Ljava/util/List;Ljava/util/List;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/util/List<Ljava/time/zone/ZoneOffsetTransition;>;Ljava/util/List<Ljava/time/zone/ZoneOffsetTransition;>;Ljava/util/List<Ljava/time/zone/ZoneOffsetTransitionRule;>;)V")]
        pub fn new_zoneof_zoneof_list_list_list(baseStandardOffset: ZoneOffset, baseWallOffset: ZoneOffset, standardOffsetTransitionList: Object, transitionList: Object, lastRules: Object) -> Result<Self> {
            panic!("stub: java/time/zone/ZoneRules.<init>:(Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/util/List;Ljava/util/List;Ljava/util/List;)V")
        }

        #[java_method(name = "<init>", descriptor = "([J[Ljava/time/ZoneOffset;[J[Ljava/time/ZoneOffset;[Ljava/time/zone/ZoneOffsetTransitionRule;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_l_arr_zon_arr_l_arr_zon_arr_zon(standardTransitions: Rc<RefCell<Vec<i64>>>, standardOffsets: Rc<RefCell<Vec<ZoneOffset>>>, savingsInstantTransitions: Rc<RefCell<Vec<i64>>>, wallOffsets: Rc<RefCell<Vec<ZoneOffset>>>, lastRules: Rc<RefCell<Vec<ZoneOffsetTransitionRule>>>) -> Result<Self> {
            panic!("stub: java/time/zone/ZoneRules.<init>:([J[Ljava/time/ZoneOffset;[J[Ljava/time/ZoneOffset;[Ljava/time/zone/ZoneOffsetTransitionRule;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/time/ZoneOffset;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_zoneof(offset: ZoneOffset) -> Result<Self> {
            panic!("stub: java/time/zone/ZoneRules.<init>:(Ljava/time/ZoneOffset;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/zone/ZoneRules.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneRules.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "writeExternal", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeExternal(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/zone/ZoneRules.writeExternal:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "readExternal", descriptor = "(Ljava/io/DataInput;)Ljava/time/zone/ZoneRules;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readExternal(in_: Object) -> Result<ZoneRules> {
            panic!("stub: java/time/zone/ZoneRules.readExternal:(Ljava/io/DataInput;)Ljava/time/zone/ZoneRules;")
        }

        #[java_method(name = "isFixedOffset", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isFixedOffset(&self) -> Result<bool> {
            panic!("stub: java/time/zone/ZoneRules.isFixedOffset:()Z")
        }

        #[java_method(name = "getOffset", descriptor = "(Ljava/time/Instant;)Ljava/time/ZoneOffset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getOffset(Ljava/time/Instant;)Ljava/time/ZoneOffset;
        pub fn getOffset_instan(&self, mut instant: Instant) -> Result<ZoneOffset> {
            let this = self;
            if ((this.__get_savingsInstantTransitions().borrow().len() as i32)==0) {
                return Ok(Clone::clone(&this.__get_wallOffsets().borrow()[0i32 as usize]));
            }
            let _t0 = instant.getEpochSecond()?;
            let mut epochSec: i64 = _t0;
            let _t1 = this.findYear(epochSec, Clone::clone(&Clone::clone(&this.__get_wallOffsets().borrow()[((this.__get_wallOffsets().borrow().len() as i32)).wrapping_sub(1i32) as usize])))?;
            let mut year: i32 = _t1;
            let _t2 = this.findTransitionArray(year)?;
            let mut transArray: Rc<RefCell<Vec<ZoneOffsetTransition>>> = _t2;
            let mut trans: Object = Object::default();
            let mut i: i32 = 0i32;
            let mut trans = Default::default();
            loop {
                if i >= (transArray.borrow().len() as i32) { break; }
                trans = Clone::clone(&transArray.borrow()[i as usize]);
                let _t3 = trans.toEpochSecond()?;
                if (((epochSec>(_t3)) as i32-((epochSec)<(_t3)) as i32)<0) {
                    let _t4 = trans.getOffsetBefore()?;
                    return Ok(_t4);
                }
                i = i.wrapping_add(1i32);
            }
            let _t3 = trans.getOffsetAfter()?;
            return Ok(_t3);
            let _t4: i32 = Arrays::binarySearch_arr_l_l(Clone::clone(&this.__get_savingsInstantTransitions()), epochSec)?;
            year = _t4;
            if (year<0) {
                year = ((year).wrapping_neg()).wrapping_sub(2i32);
            }
            Ok(Clone::clone(&this.__get_wallOffsets().borrow()[(year).wrapping_add(1i32) as usize]))
        }

        #[java_method(name = "getOffset", descriptor = "(Ljava/time/LocalDateTime;)Ljava/time/ZoneOffset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffset_locald(&self, localDateTime: LocalDateTime) -> Result<ZoneOffset> {
            panic!("stub: java/time/zone/ZoneRules.getOffset:(Ljava/time/LocalDateTime;)Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "getValidOffsets", descriptor = "(Ljava/time/LocalDateTime;)Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/time/LocalDateTime;)Ljava/util/List<Ljava/time/ZoneOffset;>;")]
        pub fn getValidOffsets(&self, localDateTime: LocalDateTime) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneRules.getValidOffsets:(Ljava/time/LocalDateTime;)Ljava/util/List;")
        }

        #[java_method(name = "getTransition", descriptor = "(Ljava/time/LocalDateTime;)Ljava/time/zone/ZoneOffsetTransition;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTransition(&self, localDateTime: LocalDateTime) -> Result<ZoneOffsetTransition> {
            panic!("stub: java/time/zone/ZoneRules.getTransition:(Ljava/time/LocalDateTime;)Ljava/time/zone/ZoneOffsetTransition;")
        }

        #[java_method(name = "getOffsetInfo", descriptor = "(Ljava/time/LocalDateTime;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsetInfo(&self, dt: LocalDateTime) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneRules.getOffsetInfo:(Ljava/time/LocalDateTime;)Ljava/lang/Object;")
        }

        #[java_method(name = "findOffsetInfo", descriptor = "(Ljava/time/LocalDateTime;Ljava/time/zone/ZoneOffsetTransition;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findOffsetInfo(&self, dt: LocalDateTime, trans: ZoneOffsetTransition) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneRules.findOffsetInfo:(Ljava/time/LocalDateTime;Ljava/time/zone/ZoneOffsetTransition;)Ljava/lang/Object;")
        }

        #[java_method(name = "findTransitionArray", descriptor = "(I)[Ljava/time/zone/ZoneOffsetTransition;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findTransitionArray(&self, mut year: i32) -> Result<Rc<RefCell<Vec<ZoneOffsetTransition>>>> {
            let this = self;
            let mut yearObj: i32 = year;
            let _vdispatch0: Object = if let Some(_d) = this.__get_lastRulesCache().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.get(yearObj.into())? } else if let Some(_d) = this.__get_lastRulesCache().0.as_any().downcast_ref::<Object>() { _d.get(yearObj.into())? } else if let Some(__f) = this.__get_lastRulesCache().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(yearObj.into())? } else { Default::default() };
            let mut transArray = (_vdispatch0).downcast::<Rc<RefCell<Vec<ZoneOffsetTransition>>>>();
            if !_is_jnull(&transArray) {
                return Ok(transArray);
            }
            let mut ruleArray = this.__get_lastRules();
            let mut _arr1: Rc<RefCell<Vec<ZoneOffsetTransition>>> = Rc::new(RefCell::new(vec![Default::default(); (ruleArray.borrow().len() as i32) as usize]));
            transArray = _arr1;
            let mut i: i32 = 0i32;
            loop {
                if i >= (ruleArray.borrow().len() as i32) { break; }
                let _t2 = Clone::clone(&ruleArray.borrow()[i as usize]).createTransition(year)?;
                transArray.borrow_mut()[i as usize] = Clone::clone(&_t2);
                i = i.wrapping_add(1i32);
            }
            if year < 2100i32 {
                let _vdispatch2: Object = if let Some(_d) = this.__get_lastRulesCache().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.putIfAbsent(yearObj.into(), Object::from_any(transArray.clone()))? } else if let Some(_d) = this.__get_lastRulesCache().0.as_any().downcast_ref::<Object>() { _d.putIfAbsent(yearObj.into(), Object::from_any(transArray.clone()))? } else if let Some(__f) = this.__get_lastRulesCache().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>>>() { (__f)(yearObj.into(), Object::from_any(transArray.clone()))? } else { Default::default() };
            }
            Ok(transArray)
        }

        #[java_method(name = "getStandardOffset", descriptor = "(Ljava/time/Instant;)Ljava/time/ZoneOffset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getStandardOffset(&self, mut instant: Instant) -> Result<ZoneOffset> {
            let this = self;
            if ((this.__get_standardTransitions().borrow().len() as i32)==0) {
                return Ok(Clone::clone(&this.__get_standardOffsets().borrow()[0i32 as usize]));
            }
            let _t0 = instant.getEpochSecond()?;
            let mut epochSec: i64 = _t0;
            let _t1: i32 = Arrays::binarySearch_arr_l_l(Clone::clone(&this.__get_standardTransitions()), epochSec)?;
            let mut index: i32 = _t1;
            if (index<0) {
                index = ((index).wrapping_neg()).wrapping_sub(2i32);
            }
            Ok(Clone::clone(&this.__get_standardOffsets().borrow()[(index).wrapping_add(1i32) as usize]))
        }

        #[java_method(name = "getDaylightSavings", descriptor = "(Ljava/time/Instant;)Ljava/time/Duration;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDaylightSavings(&self, instant: Instant) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneRules.getDaylightSavings:(Ljava/time/Instant;)Ljava/time/Duration;")
        }

        #[java_method(name = "isDaylightSavings", descriptor = "(Ljava/time/Instant;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDaylightSavings(&self, mut instant: Instant) -> Result<bool> {
            let this = self;
            let _t0 = this.getStandardOffset(Clone::clone(&instant))?;
            let _t1 = this.getOffset_instan(Clone::clone(&instant))?;
            let _t2 = _t0.equals(Object::from_any(_t1.clone()))?;
            Ok(!(_t2))
        }

        #[java_method(name = "isValidOffset", descriptor = "(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValidOffset(&self, localDateTime: LocalDateTime, offset: ZoneOffset) -> Result<bool> {
            panic!("stub: java/time/zone/ZoneRules.isValidOffset:(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;)Z")
        }

        #[java_method(name = "nextTransition", descriptor = "(Ljava/time/Instant;)Ljava/time/zone/ZoneOffsetTransition;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextTransition(&self, instant: Instant) -> Result<ZoneOffsetTransition> {
            panic!("stub: java/time/zone/ZoneRules.nextTransition:(Ljava/time/Instant;)Ljava/time/zone/ZoneOffsetTransition;")
        }

        #[java_method(name = "previousTransition", descriptor = "(Ljava/time/Instant;)Ljava/time/zone/ZoneOffsetTransition;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn previousTransition(&self, instant: Instant) -> Result<ZoneOffsetTransition> {
            panic!("stub: java/time/zone/ZoneRules.previousTransition:(Ljava/time/Instant;)Ljava/time/zone/ZoneOffsetTransition;")
        }

        #[java_method(name = "findYear", descriptor = "(JLjava/time/ZoneOffset;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findYear(&self, mut epochSecond: i64, mut offset: ZoneOffset) -> Result<i32> {
            let this = self;
            let _t0 = offset.getTotalSeconds()?;
            let mut localSecond = (epochSecond).wrapping_add((_t0 as i64));
            let _t1: i64 = Math::floorDiv_l_i(localSecond, 331i32)?;
            let mut zeroDay = (_t1).wrapping_add(719528i64);
            zeroDay = (zeroDay).wrapping_sub(60i64);
            let mut adjust: i64 = 0i64;
            if (((zeroDay>(0i64)) as i32-((zeroDay)<(0i64)) as i32)<0) {
                let mut adjustCycles = (((zeroDay).wrapping_add(1i64)/146097i64)).wrapping_sub(1i64);
                adjust = (adjustCycles).wrapping_mul(400i64);
                zeroDay = (zeroDay).wrapping_add(((adjustCycles).wrapping_neg()).wrapping_mul(146097i64));
            }
            let mut adjustCycles = (((400i64).wrapping_mul(zeroDay)).wrapping_add(591i64)/146097i64);
            let mut doyEst = (zeroDay).wrapping_sub(((((365i64).wrapping_mul(adjustCycles)).wrapping_add((adjustCycles/4i64))).wrapping_sub((adjustCycles/100i64))).wrapping_add((adjustCycles/400i64)));
            if (((doyEst>(0i64)) as i32-((doyEst)<(0i64)) as i32)<0) {
                adjustCycles = (adjustCycles).wrapping_sub(1i64);
                doyEst = (zeroDay).wrapping_sub(((((365i64).wrapping_mul(adjustCycles)).wrapping_add((adjustCycles/4i64))).wrapping_sub((adjustCycles/100i64))).wrapping_add((adjustCycles/400i64)));
            }
            adjustCycles = (adjustCycles).wrapping_add(adjust);
            if (((doyEst>(306i64)) as i32-((doyEst)<(306i64)) as i32)>=0) {
                adjustCycles = (adjustCycles).wrapping_add(1i64);
            }
            let _t2: i64 = Math::min_l_l(adjustCycles, 999999999i64)?;
            Ok((_t2 as i32))
        }

        #[java_method(name = "getTransitions", descriptor = "()Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<Ljava/time/zone/ZoneOffsetTransition;>;")]
        pub fn getTransitions(&self) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneRules.getTransitions:()Ljava/util/List;")
        }

        #[java_method(name = "getTransitionRules", descriptor = "()Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<Ljava/time/zone/ZoneOffsetTransitionRule;>;")]
        pub fn getTransitionRules(&self) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneRules.getTransitionRules:()Ljava/util/List;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, otherRules: Object) -> Result<bool> {
            panic!("stub: java/time/zone/ZoneRules.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }
    }
}
