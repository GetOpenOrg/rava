#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/reflect/Modifier",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "Modifier.java",
))]
pub struct Modifier;

impl Modifier {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this)
    }

    // java: isPublic(I)Z
    pub fn isPublic(mod_: i32) -> Result<bool> {
        Ok((mod_&1i32)!=0i32)
    }

    // java: isPrivate(I)Z
    pub fn isPrivate(mod_: i32) -> Result<bool> {
        Ok((mod_&2i32)!=0i32)
    }

    // java: isProtected(I)Z
    pub fn isProtected(mod_: i32) -> Result<bool> {
        Ok((mod_&4i32)!=0i32)
    }

    // java: isStatic(I)Z
    pub fn isStatic(mod_: i32) -> Result<bool> {
        Ok((mod_&8i32)!=0i32)
    }

    // java: isFinal(I)Z
    pub fn isFinal(mod_: i32) -> Result<bool> {
        Ok((mod_&16i32)!=0i32)
    }

    // java: isSynchronized(I)Z
    pub fn isSynchronized(mod_: i32) -> Result<bool> {
        Ok((mod_&32i32)!=0i32)
    }

    // java: isVolatile(I)Z
    pub fn isVolatile(mod_: i32) -> Result<bool> {
        Ok((mod_&64i32)!=0i32)
    }

    // java: isTransient(I)Z
    pub fn isTransient(mod_: i32) -> Result<bool> {
        Ok((mod_&128i32)!=0i32)
    }

    // java: isNative(I)Z
    pub fn isNative(mod_: i32) -> Result<bool> {
        Ok((mod_&256i32)!=0i32)
    }

    // java: isInterface(I)Z
    pub fn isInterface(mod_: i32) -> Result<bool> {
        Ok((mod_&512i32)!=0i32)
    }

    // java: isAbstract(I)Z
    pub fn isAbstract(mod_: i32) -> Result<bool> {
        Ok((mod_&1024i32)!=0i32)
    }

    // java: isStrict(I)Z
    pub fn isStrict(mod_: i32) -> Result<bool> {
        Ok((mod_&2048i32)!=0i32)
    }

    // java: toString(I)Ljava/lang/String;
    pub fn toString(mod_: i32) -> Result<String> {
        let mut sj: StringJoiner = StringJoiner::new(String::from(""))?;
        let _t0 = sj.add(String::from("public"))?;
        let _t1 = sj.add(String::from("protected"))?;
        let _t2 = sj.add(String::from("private"))?;
        let _t3 = sj.add(String::from("abstract"))?;
        let _t4 = sj.add(String::from("static"))?;
        let _t5 = sj.add(String::from("final"))?;
        let _t6 = sj.add(String::from("transient"))?;
        let _t7 = sj.add(String::from("volatile"))?;
        let _t8 = sj.add(String::from("synchronized"))?;
        let _t9 = sj.add(String::from("native"))?;
        let _t10 = sj.add(String::from("strictfp"))?;
        let _t11 = sj.add(String::from("interface"))?;
        let _t12 = sj.toString()?;
        Ok(_t12)
    }

    // java: isSynthetic(I)Z
    pub fn isSynthetic(mod_: i32) -> Result<bool> {
        Ok((mod_&4096i32)!=0i32)
    }

    // java: isMandated(I)Z
    pub fn isMandated(mod_: i32) -> Result<bool> {
        Ok((mod_&32768i32)!=0i32)
    }

    // java: classModifiers()I
    pub fn classModifiers() -> Result<i32> {
        Ok(3103i32)
    }

    // java: interfaceModifiers()I
    pub fn interfaceModifiers() -> Result<i32> {
        Ok(3087i32)
    }

    // java: constructorModifiers()I
    pub fn constructorModifiers() -> Result<i32> {
        Ok(7i32)
    }

    // java: methodModifiers()I
    pub fn methodModifiers() -> Result<i32> {
        Ok(3391i32)
    }

    // java: fieldModifiers()I
    pub fn fieldModifiers() -> Result<i32> {
        Ok(223i32)
    }

    // java: parameterModifiers()I
    pub fn parameterModifiers() -> Result<i32> {
        Ok(16i32)
    }
}
