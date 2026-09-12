#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/System$Logger$Level",
    super_class = "java/lang/Enum",
    interfaces  = "",
    access      = "public final",
    source      = "System.java",
))]
pub struct System_Logger_Level {
    #[cfg_attr(any(), java_field(name = "severity", descriptor = "I", access = "private final"))]
    pub severity: Field<i32>,
}

impl System_Logger_Level {
    // java: values()[Ljava/lang/System$Logger$Level;
    pub fn values() -> Result<Vec<Object>> {
        let _t0 = System_Logger_Level::_VALUES().clone()?;
        Ok(_t0)
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/System$Logger$Level;
    pub fn valueOf(name: String) -> Result<Object> {
        let _t0: Object = Enum::valueOf(1i32, name)?;
        Ok(_t0)
    }

    // java: <init>(Ljava/lang/String;II)V
    pub fn new(arg_0: String, arg_1: i32, severity: i32) -> Result<Self> {
        let this = Self { severity: Field::new(0) };
        /* invokespecial Method java/lang/Enum.<init>:(Ljava/lang/String;I)V */
        this.severity.set(severity);
        Ok(this)
    }

    // java: getName()Ljava/lang/String;
    pub fn getName(&self) -> Result<String> {
        let this = self;
        let _t0 = this.name()?;
        Ok(_t0)
    }

    // java: getSeverity()I
    pub fn getSeverity(&self) -> Result<i32> {
        let this = self;
        Ok(this.severity.get())
    }
}
