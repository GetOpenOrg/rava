#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/ProcessEnvironment",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "ProcessEnvironment.java",
))]
pub struct ProcessEnvironment;

impl ProcessEnvironment {
    #[cfg_attr(any(), java_method(name = "getenv", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "static"))]
    // java: getenv(Ljava/lang/String;)Ljava/lang/String;
    pub fn getenv__str(name: String) -> Result<String> {
        let _t0 = ProcessEnvironment::theUnmodifiableEnvironment().get(name)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getenv", descriptor = "()Ljava/util/Map;", access = "static"))]
    // java: getenv()Ljava/util/Map;
    pub fn getenv() -> Result<Object> {
        Ok(ProcessEnvironment::theUnmodifiableEnvironment())
    }

    #[cfg_attr(any(), java_method(name = "environment", descriptor = "()Ljava/util/Map;", access = "static"))]
    pub fn environment() -> Result<Object> {
        let _t0 = ProcessEnvironment::theEnvironment().clone()?;
        Ok(ProcessEnvironment_StringEnvironment::new(_t0)?)
    }

    #[cfg_attr(any(), java_method(name = "emptyEnvironment", descriptor = "(I)Ljava/util/Map;", access = "static"))]
    pub fn emptyEnvironment(capacity: i32) -> Result<Object> {
        Ok(ProcessEnvironment_StringEnvironment::new(HashMap::<_, _>::new()?)?)
    }

    #[cfg_attr(any(), java_native(name = "environ", descriptor = "()[[B", access = "private static native"))]
    pub fn environ() -> Result<Vec<Vec<i8>>> {
        todo!("native java/lang/ProcessEnvironment.environ")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "validateVariable", descriptor = "(Ljava/lang/String;)V", access = "private static"))]
    pub fn validateVariable(name: String) -> Result<()> {
        let _t0 = name.indexOf(61i32)?;
        let _t1 = name.indexOf(0i32)?;
        String::new().append(&String::from("Invalid environment variable name: ""))?;
        String::new().append(&name)?;
        String::new().append(&String::from("""))?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "validateValue", descriptor = "(Ljava/lang/String;)V", access = "private static"))]
    pub fn validateValue(value: String) -> Result<()> {
        let _t0 = value.indexOf(0i32)?;
        String::new().append(&String::from("Invalid environment variable value: ""))?;
        String::new().append(&value)?;
        String::new().append(&String::from("""))?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "toEnvironmentBlock", descriptor = "(Ljava/util/Map;[I)[B", access = "static"))]
    pub fn toEnvironmentBlock(map: Object, envc: &[i32]) -> Result<Vec<i8>> {
        /* TODO: aconst_null  */
        let _t0 = map.toEnvironmentBlock(envc)?;
        Ok(_t0)
    }
}
