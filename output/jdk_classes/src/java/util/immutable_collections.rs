#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections;

impl ImmutableCollections {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: uoe()Ljava/lang/UnsupportedOperationException;
    pub fn uoe() -> Result<Object> {
        Ok(UnsupportedOperationException::new()?)
    }

    // java: listCopy(Ljava/util/Collection;)Ljava/util/List;
    pub fn listCopy(coll: Object) -> Result<Object> {
        let mut c: Object = coll;
        return Ok(coll);
        let _t0 = coll.isEmpty()?;
        let _t1: Object = List::of()?;
        return Ok(_t1);
        let _t2 = coll.toArray()?;
        let _t3: Object = List::of__arr_obj(&_t2)?;
        Ok(_t3)
    }

    // java: listFromArray([Ljava/lang/Object;)Ljava/util/List;
    pub fn listFromArray(input: &[Object]) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity((input.len() as i32) as usize);
        let mut tmp: Vec<Object> = _arr0;
        let mut i: i32 = 0i32;
        loop {
            if i >= (input.len() as i32) { break; }
            let _t0: Object = Objects::requireNonNull__obj(input[i as usize].clone())?;
            tmp[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(ImmutableCollections_ListN::new(tmp, 0i32)?)
    }

    // java: listFromTrustedArray([Ljava/lang/Object;)Ljava/util/List;
    pub fn listFromTrustedArray(input: &[Object]) -> Result<Object> {
        let _t0 = input.getClass()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut local_1: Vec<Object> = input;
        let mut local_2: i32 = (local_1.len() as i32);
        let mut local_3: i32 = 0i32;
        loop {
            if local_3 >= local_2 { break; }
            let mut o: Object = local_1[local_3 as usize].clone();
            let _t0: Object = Objects::requireNonNull__obj(o)?;
            local_3 = local_3.wrapping_add(1i32);
        }
        /* TODO: tableswitch default:115 low:0 high:2 */
        Ok(ImmutableCollections_ListN::new(input, 0i32)?)
    }

    // java: listFromTrustedArrayNullsAllowed([Ljava/lang/Object;)Ljava/util/List;
    pub fn listFromTrustedArrayNullsAllowed(input: &[Object]) -> Result<Object> {
        let _t0 = input.getClass()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(ImmutableCollections::EMPTY_LIST_NULLS());
        Ok(ImmutableCollections_ListN::new(input, 1i32)?)
    }
}
