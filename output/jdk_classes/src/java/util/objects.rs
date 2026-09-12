#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Objects",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "Objects.java",
))]
pub struct Objects;

impl Objects {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this)
    }

    // java: equals(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn equals(a: Object, b: Object) -> Result<bool> {
        let _t0 = a.equals(b)?;
        Ok(_t0!=0i32)
    }

    // java: deepEquals(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn deepEquals(a: Object, b: Object) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let _t0: bool = Arrays::deepEquals0(a, b)?;
        Ok(_t0)
    }

    // java: hashCode(Ljava/lang/Object;)I
    pub fn hashCode(o: Object) -> Result<i32> {
        let _t0 = o.hashCode()?;
        Ok(0i32)
    }

    // java: hash([Ljava/lang/Object;)I
    pub fn hash(values: &[Object]) -> Result<i32> {
        let _t0: i32 = Arrays::hashCode__arr_obj(&values)?;
        Ok(_t0)
    }

    // java: toString(Ljava/lang/Object;)Ljava/lang/String;
    // java: toString(Ljava/lang/Object;)Ljava/lang/String;
    pub fn toString__obj(o: Object) -> Result<String> {
        Ok(String::from_owned(format!("{}", o)))
    }

    // java: toString(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;
    // java: toString(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;
    pub fn toString__obj_str(o: Object, nullDefault: String) -> Result<String> {
        let _t0 = o.toString()?;
        Ok(nullDefault)
    }

    // java: toIdentityString(Ljava/lang/Object;)Ljava/lang/String;
    pub fn toIdentityString(o: Object) -> Result<String> {
        let _t0: Object = Objects::requireNonNull__obj(o)?;
        let _t1 = o.getClass()?;
        let _t2 = _t1.getName()?;
        String::new().append(&_t2)?;
        String::new().append(&String::from("@"))?;
        let _t3: i32 = System::identityHashCode(o)?;
        let _t4: String = Integer::toHexString(_t3)?;
        String::new().append(&_t4)?;
        Ok(String::new())
    }

    // java: compare(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn compare(a: Object, b: Object, c: Object) -> Result<i32> {
        let _t0 = c.compare(a, b)?;
        Ok(_t0)
    }

    // java: requireNonNull(Ljava/lang/Object;)Ljava/lang/Object;
    // java: requireNonNull(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn requireNonNull__obj(obj: Object) -> Result<Object> {
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(obj)
    }

    // java: requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;
    // java: requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;
    pub fn requireNonNull__obj_str(obj: Object, message: String) -> Result<Object> {
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(obj)
    }

    // java: isNull(Ljava/lang/Object;)Z
    pub fn isNull(obj: Object) -> Result<bool> {
        Ok(obj.is_none())
    }

    // java: nonNull(Ljava/lang/Object;)Z
    pub fn nonNull(obj: Object) -> Result<bool> {
        Ok(!obj.is_none())
    }

    // java: requireNonNullElse(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn requireNonNullElse(obj: Object, defaultObj: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj_str(defaultObj, String::from("defaultObj"))?;
        Ok(_t0)
    }

    // java: requireNonNullElseGet(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn requireNonNullElseGet(obj: Object, supplier: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj_str(supplier, String::from("supplier"))?;
        let _t1 = _t0.get()?;
        let _t2: Object = Objects::requireNonNull__obj_str(_t1, String::from("supplier.get()"))?;
        Ok(_t2)
    }

    // java: requireNonNull(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;
    // java: requireNonNull(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn requireNonNull__obj_suppli(obj: Object, messageSupplier: Object) -> Result<Object> {
        /* TODO: aconst_null  */
        let _t0 = messageSupplier.get()?;
        /* invokespecial Method java/lang/NullPointerException.<init>:(Ljava/lang/String;)V */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(obj)
    }

    // java: checkIndex(II)I
    // java: checkIndex(II)I
    pub fn checkIndex__i_i(index: i32, length: i32) -> Result<i32> {
        /* TODO: aconst_null  */
        let _t0: i32 = Preconditions::checkIndex(todo!("stack underflow"), index, length)?;
        Ok(_t0)
    }

    // java: checkFromToIndex(III)I
    // java: checkFromToIndex(III)I
    pub fn checkFromToIndex__i_i_i(fromIndex: i32, toIndex: i32, length: i32) -> Result<i32> {
        /* TODO: aconst_null  */
        let _t0: i32 = Preconditions::checkFromToIndex(todo!("stack underflow"), fromIndex, toIndex, length)?;
        Ok(_t0)
    }

    // java: checkFromIndexSize(III)I
    // java: checkFromIndexSize(III)I
    pub fn checkFromIndexSize__i_i_i(fromIndex: i32, size: i32, length: i32) -> Result<i32> {
        /* TODO: aconst_null  */
        let _t0: i32 = Preconditions::checkFromIndexSize(todo!("stack underflow"), fromIndex, size, length)?;
        Ok(_t0)
    }

    // java: checkIndex(JJ)J
    // java: checkIndex(JJ)J
    pub fn checkIndex__l_l(index: i64, arg_1: i64) -> Result<i64> {
        /* TODO: aconst_null  */
        let _t0: i64 = Preconditions::checkIndex(todo!("stack underflow"), index, local_2)?;
        Ok(_t0)
    }

    // java: checkFromToIndex(JJJ)J
    // java: checkFromToIndex(JJJ)J
    pub fn checkFromToIndex__l_l_l(fromIndex: i64, arg_1: i64, toIndex: i64) -> Result<i64> {
        /* TODO: aconst_null  */
        let _t0: i64 = Preconditions::checkFromToIndex(todo!("stack underflow"), fromIndex, toIndex, local_4)?;
        Ok(_t0)
    }

    // java: checkFromIndexSize(JJJ)J
    // java: checkFromIndexSize(JJJ)J
    pub fn checkFromIndexSize__l_l_l(fromIndex: i64, arg_1: i64, size: i64) -> Result<i64> {
        /* TODO: aconst_null  */
        let _t0: i64 = Preconditions::checkFromIndexSize(todo!("stack underflow"), fromIndex, size, local_4)?;
        Ok(_t0)
    }
}
