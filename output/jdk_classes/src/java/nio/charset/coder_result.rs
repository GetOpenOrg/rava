#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/CoderResult",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "CoderResult.java",
))]
pub struct CoderResult {
    #[cfg_attr(any(), java_field(name = "type", descriptor = "I", access = "private final"))]
    pub type: Field<i32>,
    #[cfg_attr(any(), java_field(name = "length", descriptor = "I", access = "private final"))]
    pub length: Field<i32>,
}

impl CoderResult {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(II)V", access = "private"))]
    pub fn new(type_: i32, length: i32) -> Result<Self> {
        let this = Self { type: Field::new(0), length: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.type.set(type_);
        this.length.set(length);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut nm: String = CoderResult::names()[this.type.get() as usize].clone();
        let _t0 = this.isError()?;
        String::new().append(&nm)?;
        String::new().append(&String::from("["))?;
        String::new().append(&this.length.get())?;
        String::new().append(&String::from("]"))?;
        Ok(nm)
    }

    #[cfg_attr(any(), java_method(name = "isUnderflow", descriptor = "()Z", access = "public"))]
    pub fn isUnderflow(&self) -> Result<bool> {
        let this = self;
        Ok(this.type.get()==0i32)
    }

    #[cfg_attr(any(), java_method(name = "isOverflow", descriptor = "()Z", access = "public"))]
    pub fn isOverflow(&self) -> Result<bool> {
        let this = self;
        Ok(this.type.get() == 1i32)
    }

    #[cfg_attr(any(), java_method(name = "isError", descriptor = "()Z", access = "public"))]
    pub fn isError(&self) -> Result<bool> {
        let this = self;
        Ok(this.type.get() >= 2i32)
    }

    #[cfg_attr(any(), java_method(name = "isMalformed", descriptor = "()Z", access = "public"))]
    pub fn isMalformed(&self) -> Result<bool> {
        let this = self;
        Ok(this.type.get() == 2i32)
    }

    #[cfg_attr(any(), java_method(name = "isUnmappable", descriptor = "()Z", access = "public"))]
    pub fn isUnmappable(&self) -> Result<bool> {
        let this = self;
        Ok(this.type.get() == 3i32)
    }

    #[cfg_attr(any(), java_method(name = "length", descriptor = "()I", access = "public"))]
    pub fn length(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.isError()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(this.length.get())
    }

    #[cfg_attr(any(), java_method(name = "malformedForLength", descriptor = "(I)Ljava/nio/charset/CoderResult;", access = "public static"))]
    pub fn malformedForLength(length: i32) -> Result<Object> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(CoderResult::malformed4()[(length).wrapping_sub(1i32) as usize].clone());
        /* TODO: invokedynamic 72 */
        let _t0 = 4i32.computeIfAbsent(CoderResult$Cache::INSTANCE().malformed.get(), length)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "unmappableForLength", descriptor = "(I)Ljava/nio/charset/CoderResult;", access = "public static"))]
    pub fn unmappableForLength(length: i32) -> Result<Object> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(CoderResult::unmappable4()[(length).wrapping_sub(1i32) as usize].clone());
        /* TODO: invokedynamic 88 */
        let _t0 = 4i32.computeIfAbsent(CoderResult$Cache::INSTANCE().unmappable.get(), length)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "throwException", descriptor = "()V", access = "public"))]
    pub fn throwException(&self) -> Result<()> {
        let this = self;
        /* TODO: tableswitch default:76 low:0 high:3 */
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }
}
