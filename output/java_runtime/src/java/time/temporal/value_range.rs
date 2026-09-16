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
    #[binary_name       = "java/time/temporal/ValueRange"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ValueRange.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/time/temporal/ValueRange"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ValueRange {
        #[cfg_attr(any(), java_field(name = "minSmallest", descriptor = "J", access = "private", modifiers = "final", is_static = false))]
        pub minSmallest: i64,
        #[cfg_attr(any(), java_field(name = "minLargest", descriptor = "J", access = "private", modifiers = "final", is_static = false))]
        pub minLargest: i64,
        #[cfg_attr(any(), java_field(name = "maxSmallest", descriptor = "J", access = "private", modifiers = "final", is_static = false))]
        pub maxSmallest: i64,
        #[cfg_attr(any(), java_field(name = "maxLargest", descriptor = "J", access = "private", modifiers = "final", is_static = false))]
        pub maxLargest: i64,
    }

    impl ValueRange {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-7317881728594519368"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -7317881728594519368i64
        }

        #[java_method(name = "of", descriptor = "(JJ)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_l_l(min: i64, arg1: i64) -> Result<ValueRange> {
            panic!("stub: java/time/temporal/ValueRange.of:(JJ)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "of", descriptor = "(JJJ)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_l_l_l(min: i64, arg1: i64, maxSmallest: i64) -> Result<ValueRange> {
            panic!("stub: java/time/temporal/ValueRange.of:(JJJ)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "of", descriptor = "(JJJJ)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_l_l_l_l(minSmallest: i64, arg1: i64, minLargest: i64, arg3: i64) -> Result<ValueRange> {
            panic!("stub: java/time/temporal/ValueRange.of:(JJJJ)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "<init>", descriptor = "(JJJJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(minSmallest: i64, arg1: i64, minLargest: i64, arg3: i64) -> Result<Self> {
            panic!("stub: java/time/temporal/ValueRange.<init>:(JJJJ)V")
        }

        #[java_method(name = "isFixed", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isFixed(&self) -> Result<bool> {
            panic!("stub: java/time/temporal/ValueRange.isFixed:()Z")
        }

        #[java_method(name = "getMinimum", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinimum(&self) -> Result<i64> {
            let this = self;
            Ok(this.__get_minSmallest())
        }

        #[java_method(name = "getLargestMinimum", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLargestMinimum(&self) -> Result<i64> {
            panic!("stub: java/time/temporal/ValueRange.getLargestMinimum:()J")
        }

        #[java_method(name = "getSmallestMaximum", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSmallestMaximum(&self) -> Result<i64> {
            panic!("stub: java/time/temporal/ValueRange.getSmallestMaximum:()J")
        }

        #[java_method(name = "getMaximum", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMaximum(&self) -> Result<i64> {
            let this = self;
            Ok(this.__get_maxLargest())
        }

        #[java_method(name = "isIntValue", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIntValue(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.getMinimum()?;
            let mut _merged2: bool;
            if (((_t0>(-2147483648i64)) as i32-((_t0)<(-2147483648i64)) as i32)>=0) {
                let _t1 = this.getMaximum()?;
                _merged2 = (((_t1>(2147483647i64)) as i32-((_t1)<(2147483647i64)) as i32)<=0);
            } else {
                _merged2 = (0i32 != 0);
            }
            Ok(_merged2)
        }

        #[java_method(name = "isValidValue", descriptor = "(J)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValidValue(&self, mut value: i64) -> Result<bool> {
            let this = self;
            let _t0 = this.getMinimum()?;
            let mut _merged2: bool;
            if (((value>(_t0)) as i32-((value)<(_t0)) as i32)>=0) {
                let _t1 = this.getMaximum()?;
                _merged2 = (((value>(_t1)) as i32-((value)<(_t1)) as i32)<=0);
            } else {
                _merged2 = (0i32 != 0);
            }
            Ok(_merged2)
        }

        #[java_method(name = "isValidIntValue", descriptor = "(J)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValidIntValue(&self, value: i64) -> Result<bool> {
            panic!("stub: java/time/temporal/ValueRange.isValidIntValue:(J)Z")
        }

        #[java_method(name = "checkValidValue", descriptor = "(JLjava/time/temporal/TemporalField;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkValidValue(&self, mut value: i64, mut field: Object) -> Result<i64> {
            let this = self;
            let _t0 = this.isValidValue(value)?;
            if !(_t0) {
                let _t1 = this.genInvalidFieldMessage(Clone::clone(&field), value)?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(value)
        }

        #[java_method(name = "checkValidIntValue", descriptor = "(JLjava/time/temporal/TemporalField;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkValidIntValue(&self, value: i64, arg1: Object) -> Result<i32> {
            panic!("stub: java/time/temporal/ValueRange.checkValidIntValue:(JLjava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "genInvalidFieldMessage", descriptor = "(Ljava/time/temporal/TemporalField;J)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn genInvalidFieldMessage(&self, mut field: Object, mut value: i64) -> Result<String> {
            let this = self;
            if !_is_jnull(&field) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Invalid value for ")))?;
                let _t1 = _t0.append_obj(Clone::clone(&field))?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" (valid values ")))?;
                let _t3 = _t2.append_obj(Object::from_any(Clone::clone(self)))?;
                let _t4 = _t3.append_str(Clone::clone(&String::from("): ")))?;
                let _t5 = _t4.append_l(value)?;
                let _t6 = _t5.toString()?;
                return Ok(_t6);
            }
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Invalid value (valid values ")))?;
            let _t1 = _t0.append_obj(Object::from_any(Clone::clone(self)))?;
            let _t2 = _t1.append_str(Clone::clone(&String::from("): ")))?;
            let _t3 = _t2.append_l(value)?;
            let _t4 = _t3.toString()?;
            Ok(_t4)
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException,java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/temporal/ValueRange.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/time/temporal/ValueRange.equals:(Ljava/lang/Object;)Z")
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
